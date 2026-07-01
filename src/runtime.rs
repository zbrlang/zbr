use crate::ast::Node;
use crate::context::{ DiscordContext, EvalResult, FnMeta, FnOutput };
use crate::functions;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;

thread_local! {
    pub static CURRENT_LOCATION: RefCell<(String, usize)> = RefCell::new((String::new(), 0));
}

/// Groups physical lines into logical statements by tracking brace depth.
/// Lines inside unclosed braces are joined with `\n` so multiline arguments
/// are preserved as a single statement.
fn group_statements(code: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() && depth == 0 {
            continue;
        }
        if trimmed.starts_with("//") && depth == 0 {
            continue;
        }

        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(trimmed);

        for ch in trimmed.chars() {
            match ch {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                }
                _ => {}
            }
        }

        if depth <= 0 {
            let stmt = current.trim().to_string();
            if !stmt.is_empty() {
                statements.push(stmt);
            }
            current.clear();
            depth = 0;
        }
    }
    if !current.is_empty() {
        let stmt = current.trim().to_string();
        if !stmt.is_empty() {
            statements.push(stmt);
        }
    }
    statements
}

pub struct Runtime {
    registry: HashMap<String, FnMeta>,
    aliases: HashMap<String, Node>,
    pub context: DiscordContext,
    pub should_reply: bool,
    pub call_depth: usize,
}

impl Runtime {
    pub fn new(context: DiscordContext) -> Self {
        let mut registry = HashMap::new();
        functions::register(&mut registry);
        Runtime {
            registry,
            aliases: HashMap::new(),
            context,
            should_reply: false,
            call_depth: 0,
        }
    }

    pub fn run(&mut self, node: Node) -> EvalResult {
        self.should_reply = false;
        let mut output = Vec::new();
        let mut fatal_error: Option<String> = None;

        match self.evaluate(node) {
            Ok(fn_output) =>
                match fn_output {
                    FnOutput::Text(t) => output.push(t),
                    FnOutput::Reply => {
                        self.should_reply = true;
                    }
                    FnOutput::Empty => {}
                    FnOutput::Error(e) => {
                        fatal_error = Some(e);
                    }
                    FnOutput::UserError(e) => {
                        if !e.is_empty() {
                            fatal_error = Some(e);
                        }
                    }
                }
            Err(e) => {
                fatal_error = Some(e);
            }
        }

        // On error: check for suppression first, then wipe state.
        if let Some(err) = fatal_error {
            let suppress_text = tokio::task::block_in_place(|| {
                tokio::runtime::Handle
                    ::current()
                    .block_on(async { self.context.suppress_error_text.lock().await.clone() })
            });
            let suppress_embed = tokio::task::block_in_place(|| {
                tokio::runtime::Handle
                    ::current()
                    .block_on(async { *self.context.suppress_error_embed.lock().await })
            });

            // If suppression is active, return the suppressed response instead of the error
            if suppress_text.is_some() || suppress_embed.is_some() {
                let embeds = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle
                        ::current()
                        .block_on(async { self.context.embed.lock().await.clone() })
                });
                let ephemeral = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle
                        ::current()
                        .block_on(async { *self.context.ephemeral.lock().await })
                });
                let use_channel = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle
                        ::current()
                        .block_on(async { self.context.use_channel.lock().await.clone() })
                });

                // Build consumed_embeds: everything EXCEPT the suppress embed
                let mut consumed = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle
                        ::current()
                        .block_on(async { self.context.consumed_embeds.lock().await.clone() })
                });
                // Mark all embeds as consumed except the one we want to show
                for i in 0..embeds.len() {
                    if Some(i) != suppress_embed {
                        consumed.insert(i);
                    }
                }

                let out_text = match suppress_text {
                    Some(ref t) if !t.is_empty() => vec![t.clone()],
                    _ => vec![],
                };

                return EvalResult {
                    output: out_text,
                    should_reply: false,
                    errors: vec![],
                    embeds,
                    consumed_embeds: consumed,
                    ephemeral,
                    use_channel,
                    components: crate::context::ComponentState::default(),
                };
            }

            // No suppression — wipe state and return the error
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    self.context.embed.lock().await.clear();
                    self.context.consumed_embeds.lock().await.clear();
                })
            });
            return EvalResult {
                output: vec![],
                should_reply: false,
                errors: vec![err],
                embeds: vec![],
                consumed_embeds: HashSet::new(),
                ephemeral: false,
                use_channel: None,
                components: crate::context::ComponentState::default(),
            };
        }

        let embeds = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { self.context.embed.lock().await.clone() })
        });

        let consumed_embeds = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { self.context.consumed_embeds.lock().await.clone() })
        });

        let ephemeral = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { *self.context.ephemeral.lock().await })
        });

        let use_channel = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { self.context.use_channel.lock().await.clone() })
        });

        let components = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { self.context.components.lock().await.clone() })
        });

        EvalResult {
            output,
            should_reply: self.should_reply,
            errors: vec![],
            embeds,
            consumed_embeds,
            ephemeral,
            use_channel,
            components,
        }
    }

    pub fn evaluate(&mut self, node: Node) -> Result<FnOutput, String> {
        self.call_depth += 1;
        let result = if self.call_depth > 100 {
            Ok(
                FnOutput::Error(
                    "Maximum call depth of 100 exceeded. Check for circular function calls.".to_string()
                )
            )
        } else {
            self.evaluate_inner(node)
        };
        self.call_depth -= 1;
        result
    }

    pub fn evaluate_inner(&mut self, node: Node) -> Result<FnOutput, String> {
        match node {
            Node::StringLiteral(s) => { Ok(FnOutput::Text(s)) }
            Node::Concat(segments) => {
                let mut result = String::new();
                for segment in segments {
                    match self.evaluate_inner(segment)? {
                        FnOutput::Text(t) => result.push_str(&t),
                        FnOutput::Reply => {
                            self.should_reply = true;
                        }
                        FnOutput::Empty => {}
                        FnOutput::Error(e) => {
                            return Ok(FnOutput::Error(e));
                        }
                        FnOutput::UserError(e) => {
                            return Ok(FnOutput::UserError(e));
                        }
                    }
                }
                Ok(FnOutput::Text(result))
            }
            Node::FunctionCall { name, args } => {
                // Check if this name is an alias
                if let Some(aliased_node) = self.aliases.get(&name).cloned() {
                    match aliased_node {
                        Node::FunctionCall { name: aliased_name, args: aliased_args } => {
                            let mut merged_args = aliased_args;
                            merged_args.extend(args);
                            return self.evaluate_inner(Node::FunctionCall {
                                name: aliased_name,
                                args: merged_args,
                            });
                        }
                        _ => {
                            return self.evaluate_inner(aliased_node);
                        }
                    }
                }

                // ── Lazy special cases
                if name == "if" {
                    return self.evaluate_if(args);
                }

                if name == "tryRun" {
                    return self.evaluate_try_run(args);
                }

                if name == "delay" {
                    return self.evaluate_delay(args);
                }

                if name == "repeat" {
                    return self.evaluate_repeat(args);
                }

                if name == "forSplit" {
                    return self.evaluate_for_split(args);
                }

                if name == "forJson" {
                    return self.evaluate_for_json(args);
                }

                if name == "async" {
                    return self.evaluate_async(args);
                }

                if name == "await" {
                    return self.evaluate_await(args);
                }

                if name == "alias" {
                    return self.evaluate_alias(args);
                }

                if name == "eval" {
                    let mut resolved = Vec::new();
                    for arg in args {
                        match self.evaluate_inner(arg)? {
                            FnOutput::Text(t) => resolved.push(t),
                            FnOutput::Reply => {
                                return Ok(FnOutput::Reply);
                            }
                            FnOutput::Empty => resolved.push(String::new()),
                            FnOutput::Error(e) => {
                                return Ok(FnOutput::Error(e));
                            }
                            FnOutput::UserError(e) => {
                                return Ok(FnOutput::UserError(e));
                            }
                        }
                    }
                    let code = resolved.into_iter().next().unwrap_or_default();
                    if code.is_empty() {
                        return Ok(FnOutput::Empty);
                    }
                    let mut eval_output: Vec<String> = Vec::new();
                    let stmts = group_statements(&code);
                    for stmt in stmts {
                        // Crucial: use parse_line and evaluate sequentially for each statement
                        if let Some(node) = crate::parser::parse_line(&stmt, Some(&self.registry)) {
                            match self.evaluate_inner(node)? {
                                FnOutput::Text(t) => eval_output.push(t),
                                FnOutput::Reply => {
                                    return Ok(FnOutput::Reply);
                                }
                                FnOutput::Empty => {}
                                FnOutput::Error(e) => {
                                    return Ok(FnOutput::Error(e));
                                }
                                FnOutput::UserError(e) => {
                                    return Ok(FnOutput::UserError(e));
                                }
                            }
                        }
                    }
                    return Ok(
                        if eval_output.is_empty() {
                            FnOutput::Empty
                        } else {
                            FnOutput::Text(eval_output.join("\n"))
                        }
                    );
                }

                // ── Normal eager evaluation ───────────────────────────────────
                let mut resolved = Vec::new();
                for arg in args {
                    match self.evaluate_inner(arg)? {
                        FnOutput::Text(t) => resolved.push(t),
                        FnOutput::Reply => {
                            return Ok(FnOutput::Reply);
                        }
                        FnOutput::Empty => resolved.push(String::new()),
                        FnOutput::Error(e) => {
                            return Ok(FnOutput::Error(e));
                        }
                        FnOutput::UserError(e) => {
                            return Ok(FnOutput::UserError(e));
                        }
                    }
                }

                match self.registry.get(if name.starts_with('Z') { &name[1..] } else { &name }) {
                    Some(meta) => {
                        let got = resolved.len();
                        if got < meta.min_args {
                            return Err(
                                format!(
                                    "Z{} - Too few arguments, expected at least {}, got {}",
                                    name,
                                    meta.min_args,
                                    got
                                )
                            );
                        }
                        if got > meta.max_args {
                            return Err(
                                format!(
                                    "Z{} - Too many arguments, expected up to {}, got {}",
                                    name,
                                    meta.max_args,
                                    got
                                )
                            );
                        }
                        Ok((meta.func)(resolved, &self.context))
                    }
                    None => Err(format!("Z{} - Unknown function", name)),
                }
            }
        }
    }

    /// Lazy evaluation for Zif{condition;then;else?}
    /// Only the winning branch is evaluated.
    fn evaluate_if(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.len() < 2 {
            return Err(
                "Zif - Too few arguments, expected at least 2 (condition and then branch)".to_string()
            );
        }

        // Evaluate the condition arg to a string
        let condition_str = match self.evaluate_inner(args[0].clone())? {
            FnOutput::Text(t) => t,
            FnOutput::Empty => String::new(),
            FnOutput::Error(e) => {
                return Ok(FnOutput::Error(e));
            }
            FnOutput::UserError(e) => {
                return Ok(FnOutput::UserError(e));
            }
            FnOutput::Reply => {
                return Ok(FnOutput::Reply);
            }
        };

        // Evaluate the condition
        let result = crate::functions::control::helpers
            ::eval_condition(&condition_str)
            .map_err(|e| format!("Zif - {}", e))?;

        // Pick the branch to evaluate
        let branch_node = if result { args.into_iter().nth(1) } else { args.into_iter().nth(2) };

        match branch_node {
            None => Ok(FnOutput::Empty),
            Some(node) => self.evaluate_inner(node),
        }
    }

    /// Lazy evaluation for ZtryRun{code;fallback}
    /// Evaluates code; if it produces an error, evaluates fallback instead.
    fn evaluate_try_run(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.is_empty() {
            return Err("ZtryRun - code argument is required".to_string());
        }

        // Try evaluating the first arg (the code branch)
        let result = self.evaluate_inner(args[0].clone())?;

        match result {
            // Error in the code branch — run the fallback if provided
            FnOutput::Error(_) | FnOutput::UserError(_) =>
                match args.into_iter().nth(1) {
                    Some(fallback) => self.evaluate_inner(fallback),
                    None => Ok(FnOutput::Empty),
                }
            // Success — return the result as-is
            other => Ok(other),
        }
    }

    /// Lazy evaluation for Zdelay{duration;code}
    /// Spawns a background task that sleeps, then evaluates the code block.
    /// NOTE: the task is in-memory and will be cancelled on bot restart.
    fn evaluate_delay(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.is_empty() {
            return Ok(FnOutput::error("delay", "duration is required"));
        }
        if args.len() < 2 {
            return Ok(FnOutput::error("delay", "code block is required"));
        }

        let duration_str = match self.evaluate_inner(args[0].clone())? {
            FnOutput::Text(t) => t,
            FnOutput::Empty => String::new(),
            FnOutput::Error(e) => {
                return Ok(FnOutput::Error(e));
            }
            FnOutput::UserError(e) => {
                return Ok(FnOutput::UserError(e));
            }
            FnOutput::Reply => {
                return Ok(FnOutput::Reply);
            }
        };
        if duration_str.is_empty() {
            return Ok(FnOutput::error("delay", "duration is required"));
        }

        let secs: u64 = if duration_str.trim_start().starts_with('-') {
            return Ok(FnOutput::error("delay", "duration must be at least 1 second"));
        } else {
            match crate::functions::cooldown::helpers::parse_duration(duration_str.trim()) {
                Ok(s) if s >= 1 => s as u64,
                Ok(_) => {
                    return Ok(FnOutput::error("delay", "duration must be at least 1 second"));
                }
                Err(e) => {
                    if e.contains("must be greater than zero") {
                        return Ok(FnOutput::error("delay", "duration must be at least 1 second"));
                    }
                    return Ok(
                        FnOutput::error("delay", format!("invalid duration: '{}'", duration_str))
                    );
                }
            }
        };

        let ctx = self.context.clone();
        let code_node = args[1].clone();

        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
            let mut rt = Runtime::new(ctx);
            let _ = rt.evaluate_inner(code_node);
        });

        Ok(FnOutput::Empty)
    }

    /// Lazy evaluation for Zrepeat{N;code}
    /// Runs code N times. ZloopIndex{} returns the current 1-based iteration.
    /// NOTE: nested loops clobber __loop_index/__loop_value of the outer loop.
    fn evaluate_repeat(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.len() < 2 {
            return Err("Zrepeat - N and code are required".to_string());
        }

        // Evaluate N eagerly
        let n_str = match self.evaluate_inner(args[0].clone())? {
            FnOutput::Text(t) => t,
            FnOutput::Empty => String::new(),
            FnOutput::Error(e) => {
                return Ok(FnOutput::Error(e));
            }
            FnOutput::UserError(e) => {
                return Ok(FnOutput::UserError(e));
            }
            FnOutput::Reply => {
                return Ok(FnOutput::Reply);
            }
        };

        let n: usize = match n_str.trim().parse::<usize>() {
            Ok(0) | Err(_) => {
                return Ok(FnOutput::error("repeat", "N must be a positive integer"));
            }
            Ok(n) => n,
        };

        if n > 1000 {
            return Ok(FnOutput::error("repeat", "maximum loop iterations (1000) exceeded"));
        }

        let body = args.into_iter().nth(1).unwrap();
        let mut output: Vec<String> = Vec::new();

        for i in 1..=n {
            // Write loop state into temp_vars
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let mut vars = self.context.temp_vars.lock().await;
                    vars.insert("__loop_index".to_string(), i.to_string());
                })
            });

            match self.evaluate_inner(body.clone())? {
                FnOutput::Text(t) => {
                    if !t.is_empty() {
                        output.push(t);
                    }
                }
                FnOutput::Empty => {}
                FnOutput::Reply => {
                    return Ok(FnOutput::Reply);
                }
                e @ FnOutput::Error(_) => {
                    return Ok(e);
                }
                e @ FnOutput::UserError(_) => {
                    return Ok(e);
                }
            }
        }

        Ok(if output.is_empty() { FnOutput::Empty } else { FnOutput::Text(output.join("\n")) })
    }

    /// Lazy evaluation for ZforSplit{code}
    /// Iterates over the current split_text. ZloopIndex{} = position (1-based), ZloopValue{} = element.
    /// NOTE: nested loops clobber __loop_index/__loop_value of the outer loop.
    fn evaluate_for_split(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.is_empty() {
            return Err("ZforSplit - code is required".to_string());
        }

        // Snapshot split_text before the loop
        let elements: Vec<String> = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { self.context.split_text.lock().await.clone() })
        });

        if elements.is_empty() {
            return Ok(
                FnOutput::error("forSplit", "no split text available — call ZtextSplit first")
            );
        }

        let body = args.into_iter().next().unwrap();
        let mut output: Vec<String> = Vec::new();

        for (i, element) in elements.iter().enumerate() {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let mut vars = self.context.temp_vars.lock().await;
                    vars.insert("__loop_index".to_string(), (i + 1).to_string());
                    vars.insert("__loop_value".to_string(), element.clone());
                })
            });

            match self.evaluate_inner(body.clone())? {
                FnOutput::Text(t) => {
                    if !t.is_empty() {
                        output.push(t);
                    }
                }
                FnOutput::Empty => {}
                FnOutput::Reply => {
                    return Ok(FnOutput::Reply);
                }
                e @ FnOutput::Error(_) => {
                    return Ok(e);
                }
                e @ FnOutput::UserError(_) => {
                    return Ok(e);
                }
            }
        }

        Ok(if output.is_empty() { FnOutput::Empty } else { FnOutput::Text(output.join("\n")) })
    }

    /// Lazy evaluation for ZforJson{key;...;code}
    /// Iterates over a JSON array at the given key path. Last arg is the code block.
    /// ZloopIndex{} = position (1-based), ZloopValue{} = serialized element.
    /// NOTE: nested loops clobber __loop_index/__loop_value of the outer loop.
    fn evaluate_for_json(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.len() < 2 {
            return Ok(FnOutput::error("forJson", "at least one key and a code block are required"));
        }

        // All args except the last are key path — evaluate them eagerly
        let key_count = args.len() - 1;
        let mut keys: Vec<String> = Vec::new();
        for arg in args[..key_count].iter() {
            match self.evaluate_inner(arg.clone())? {
                FnOutput::Text(t) => keys.push(t),
                FnOutput::Empty => keys.push(String::new()),
                FnOutput::Error(e) => {
                    return Ok(FnOutput::Error(e));
                }
                FnOutput::UserError(e) => {
                    return Ok(FnOutput::UserError(e));
                }
                FnOutput::Reply => {
                    return Ok(FnOutput::Reply);
                }
            }
        }

        let body = args.into_iter().last().unwrap();

        // Navigate the JSON object to find the array
        let elements: Result<Vec<String>, String> = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let guard = self.context.json_object.lock().await;
                match guard.as_ref() {
                    None => Err("no JSON object — call ZjsonParse first".to_string()),
                    Some(root) => {
                        // Navigate key path by cloning at each step
                        let mut cur: serde_json::Value = root.clone();
                        for key in &keys {
                            cur = match cur {
                                serde_json::Value::Object(map) =>
                                    map
                                        .get(key.as_str())
                                        .cloned()
                                        .ok_or_else(|| "key path not found".to_string())?,
                                serde_json::Value::Array(arr) => {
                                    let i = key
                                        .parse::<usize>()
                                        .map_err(|_| "key path not found".to_string())?;
                                    arr
                                        .get(i)
                                        .cloned()
                                        .ok_or_else(|| "key path not found".to_string())?
                                }
                                _ => {
                                    return Err("key path not found".to_string());
                                }
                            };
                        }
                        match cur {
                            serde_json::Value::Array(arr) =>
                                Ok(
                                    arr
                                        .iter()
                                        .map(|v| {
                                            match v {
                                                serde_json::Value::String(s) => s.clone(),
                                                serde_json::Value::Null => String::new(),
                                                other => other.to_string(),
                                            }
                                        })
                                        .collect()
                                ),
                            _ => Err("target is not an array".to_string()),
                        }
                    }
                }
            })
        });

        let elements = match elements {
            Ok(e) => e,
            Err(e) => {
                return Ok(FnOutput::error("forJson", e));
            }
        };

        let mut output: Vec<String> = Vec::new();

        for (i, element) in elements.iter().enumerate() {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let mut vars = self.context.temp_vars.lock().await;
                    vars.insert("__loop_index".to_string(), (i + 1).to_string());
                    vars.insert("__loop_value".to_string(), element.clone());
                })
            });

            match self.evaluate_inner(body.clone())? {
                FnOutput::Text(t) => {
                    if !t.is_empty() {
                        output.push(t);
                    }
                }
                FnOutput::Empty => {}
                FnOutput::Reply => {
                    return Ok(FnOutput::Reply);
                }
                e @ FnOutput::Error(_) => {
                    return Ok(e);
                }
                e @ FnOutput::UserError(_) => {
                    return Ok(e);
                }
            }
        }

        Ok(if output.is_empty() { FnOutput::Empty } else { FnOutput::Text(output.join("\n")) })
    }

    fn evaluate_async(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.len() < 2 {
            return Err("Zasync - name and code are required".to_string());
        }
        let name_str = match self.evaluate_inner(args[0].clone())? {
            FnOutput::Text(t) => t.trim().to_string(),
            FnOutput::Empty => {
                return Err("Zasync - name cannot be empty".to_string());
            }
            FnOutput::Error(e) => {
                return Ok(FnOutput::Error(e));
            }
            FnOutput::UserError(e) => {
                return Ok(FnOutput::UserError(e));
            }
            FnOutput::Reply => {
                return Ok(FnOutput::Reply);
            }
        };
        if name_str.is_empty() {
            return Err("Zasync - name cannot be empty".to_string());
        }
        let code_node = args[1].clone();
        let ctx_clone = self.context.clone();
        let handle = tokio::spawn(async move {
            let mut rt = Runtime::new(ctx_clone);
            match rt.evaluate_inner(code_node) {
                Ok(FnOutput::Text(t)) => t,
                _ => String::new(),
            }
        });
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.context.async_tasks.lock().await.insert(name_str, handle);
            })
        });
        Ok(FnOutput::Empty)
    }

    fn evaluate_await(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.is_empty() {
            return Err("Zawait - name is required".to_string());
        }
        let name_str = match self.evaluate_inner(args[0].clone())? {
            FnOutput::Text(t) => t.trim().to_string(),
            FnOutput::Empty => {
                return Err("Zawait - name cannot be empty".to_string());
            }
            FnOutput::Error(e) => {
                return Ok(FnOutput::Error(e));
            }
            FnOutput::UserError(e) => {
                return Ok(FnOutput::UserError(e));
            }
            FnOutput::Reply => {
                return Ok(FnOutput::Reply);
            }
        };
        if name_str.is_empty() {
            return Err("Zawait - name cannot be empty".to_string());
        }
        let handle = tokio::task::block_in_place(|| {
            tokio::runtime::Handle
                ::current()
                .block_on(async { self.context.async_tasks.lock().await.remove(&name_str) })
        });
        match handle {
            None => Ok(FnOutput::error("await", format!("no async block named '{}'", name_str))),
            Some(h) => {
                let result = tokio::task
                    ::block_in_place(|| tokio::runtime::Handle::current().block_on(h))
                    .unwrap_or_default();
                Ok(if result.is_empty() { FnOutput::Empty } else { FnOutput::Text(result) })
            }
        }
    }

    /// Lazy evaluation for Zalias{expression;alias_name}
    /// Registers an alias that substitutes and evaluates the stored AST node.
    fn evaluate_alias(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.len() < 2 {
            return Ok(FnOutput::error("alias", crate::error_messages::too_few_args(2, args.len())));
        }

        // Evaluate the alias name eagerly
        let alias_name_raw = match self.evaluate_inner(args[1].clone())? {
            FnOutput::Text(t) => t.trim().to_string(),
            FnOutput::Empty => {
                return Ok(
                    FnOutput::error("alias", crate::error_messages::required(2, "alias_name"))
                );
            }
            FnOutput::Error(e) => {
                return Ok(FnOutput::Error(e));
            }
            FnOutput::UserError(e) => {
                return Ok(FnOutput::UserError(e));
            }
            FnOutput::Reply => {
                return Ok(FnOutput::Reply);
            }
        };

        if alias_name_raw.is_empty() {
            return Ok(FnOutput::error("alias", crate::error_messages::required(2, "alias_name")));
        }

        // Validate that the expression is not empty
        if let Node::StringLiteral(ref s) = args[0] {
            if s.trim().is_empty() {
                return Ok(
                    FnOutput::error("alias", crate::error_messages::required(1, "expression"))
                );
            }
        }

        // Strip 'Z' if present for consistency
        let alias_name = if alias_name_raw.starts_with('Z') {
            alias_name_raw[1..].to_string()
        } else {
            alias_name_raw
        };

        // Convert StringLiteral starting with Z to FunctionCall
        let aliased_node = if let Node::StringLiteral(ref s) = args[0] {
            if s.starts_with('Z') {
                Node::FunctionCall {
                    name: s[1..].to_string(),
                    args: Vec::new(),
                }
            } else {
                args[0].clone()
            }
        } else {
            args[0].clone()
        };

        // Validate that the aliased node is a known function
        if let Node::FunctionCall { ref name, .. } = aliased_node {
            let fn_name = if name.starts_with('Z') { &name[1..] } else { name };
            if !self.registry.contains_key(fn_name) {
                return Ok(
                    FnOutput::error("alias", crate::error_messages::not_found("function", name))
                );
            }
        }

        // Store the raw expression node
        self.aliases.insert(alias_name, aliased_node);

        Ok(FnOutput::Empty)
    }
}
