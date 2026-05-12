use std::collections::HashSet;
use std::collections::HashMap;
use crate::functions;
use crate::ast::Node;
use crate::context::{DiscordContext, EvalResult, FnMeta, FnOutput};

/// Groups physical lines into logical statements by tracking brace depth.
/// Lines inside unclosed braces are joined with `\n` so multiline arguments
/// are preserved as a single statement.
fn group_statements(code: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() && depth == 0 { continue; }
        if trimmed.starts_with("//") && depth == 0 { continue; }

        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(trimmed);

        for ch in trimmed.chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
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
    pub context: DiscordContext,
}

impl Runtime {
    pub fn new(context: DiscordContext) -> Self {
        let mut registry = HashMap::new();
        functions::register(&mut registry);
        Runtime { registry, context }
    }

    pub fn run(&mut self, code: &str) -> EvalResult {
        let mut output = Vec::new();
        let mut should_reply = false;
        let mut fatal_error: Option<String> = None;
        let mut line_num = 0usize;

        let statements = group_statements(code);

        'lines: for stmt in statements {
            line_num += 1;

            match crate::parser::parse_line(&stmt) {
                Some(node) => {
                    match self.evaluate(node) {
                        Ok(fn_output) => match fn_output {
                            FnOutput::Text(t) => output.push(t),
                            FnOutput::Reply => should_reply = true,
                            FnOutput::Empty => {}
                            FnOutput::Error(e) => {
                                fatal_error = Some(format!("Line {}: {}", line_num, e));
                                break 'lines;
                            }
                            FnOutput::UserError(e) => {
                                if e.is_empty() {
                                    // Zstop — silent halt, no error
                                    break 'lines;
                                }
                                fatal_error = Some(e);
                                break 'lines;
                            }
                        },
                        Err(e) => {
                            fatal_error = Some(format!("Line {}: {}", line_num, e));
                            break 'lines;
                        }
                    }
                }
                None => {}
            }
        }

        // On error: check for suppression first, then wipe state.
        if let Some(err) = fatal_error {
            let suppress_text = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(async { self.context.suppress_error_text.lock().await.clone() })
            });
            let suppress_embed = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(async { *self.context.suppress_error_embed.lock().await })
            });

            // If suppression is active, return the suppressed response instead of the error
            if suppress_text.is_some() || suppress_embed.is_some() {
                let embeds = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { self.context.embed.lock().await.clone() })
                });
                let ephemeral = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { *self.context.ephemeral.lock().await })
                });
                let use_channel = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { self.context.use_channel.lock().await.clone() })
                });

                // Build consumed_embeds: everything EXCEPT the suppress embed
                let mut consumed = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
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
            tokio::runtime::Handle::current().block_on(async {
                self.context.embed.lock().await.clone()
            })
        });

        let consumed_embeds = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.context.consumed_embeds.lock().await.clone()
            })
        });

        let ephemeral = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { *self.context.ephemeral.lock().await })
        });

        let use_channel = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.context.use_channel.lock().await.clone() })
        });

        let components = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.context.components.lock().await.clone() })
        });

        EvalResult { output, should_reply, errors: vec![], embeds, consumed_embeds, ephemeral, use_channel, components }
    }

    pub fn evaluate(&mut self, node: Node) -> Result<FnOutput, String> {
        match node {
            Node::StringLiteral(s) => Ok(FnOutput::Text(s)),
            Node::Concat(segments) => {
                let mut result = String::new();
                for segment in segments {
                    match self.evaluate(segment)? {
                        FnOutput::Text(t) => result.push_str(&t),
                        FnOutput::Reply => return Ok(FnOutput::Reply),
                        FnOutput::Empty => {}
                        FnOutput::Error(e) => return Ok(FnOutput::Error(e)),
                        FnOutput::UserError(e) => return Ok(FnOutput::UserError(e)),
                    }
                }
                Ok(FnOutput::Text(result))
            }
            Node::FunctionCall { name, args } => {
                // ── Lazy special cases ────────────────────────────────────────
                if name == "if" {
                    return self.evaluate_if(args);
                }

                if name == "tryRun" {
                    return self.evaluate_try_run(args);
                }

                if name == "eval" {
                    let mut resolved = Vec::new();
                    for arg in args {
                        match self.evaluate(arg)? {
                            FnOutput::Text(t) => resolved.push(t),
                            FnOutput::Reply => return Ok(FnOutput::Reply),
                            FnOutput::Empty => resolved.push(String::new()),
                            FnOutput::Error(e) => return Ok(FnOutput::Error(e)),
                            FnOutput::UserError(e) => return Ok(FnOutput::UserError(e)),
                        }
                    }
                    let code = resolved.into_iter().next().unwrap_or_default();
                    if code.is_empty() {
                        return Ok(FnOutput::Empty);
                    }
                    let mut eval_output: Vec<String> = Vec::new();
                    let stmts = group_statements(&code);
                    for stmt in stmts {
                        match crate::parser::parse_line(&stmt) {
                            Some(node) => {
                                match self.evaluate(node)? {
                                    FnOutput::Text(t) => eval_output.push(t),
                                    FnOutput::Reply => return Ok(FnOutput::Reply),
                                    FnOutput::Empty => {}
                                    FnOutput::Error(e) => return Ok(FnOutput::Error(e)),
                                    FnOutput::UserError(e) => return Ok(FnOutput::UserError(e)),
                                }
                            }
                            None => {}
                        }
                    }
                    return Ok(if eval_output.is_empty() {
                        FnOutput::Empty
                    } else {
                        FnOutput::Text(eval_output.join("\n"))
                    });
                }

                // ── Normal eager evaluation ───────────────────────────────────
                let mut resolved = Vec::new();
                for arg in args {
                    match self.evaluate(arg)? {
                        FnOutput::Text(t) => resolved.push(t),
                        FnOutput::Reply => return Ok(FnOutput::Reply),
                        FnOutput::Empty => resolved.push(String::new()),
                        FnOutput::Error(e) => return Ok(FnOutput::Error(e)),
                        FnOutput::UserError(e) => return Ok(FnOutput::UserError(e)),
                    }
                }

                match self.registry.get(&name) {
                    Some(meta) => {
                        let got = resolved.len();
                        if got < meta.min_args {
                            return Err(format!("Z{} - Too few arguments, expected at least {}, got {}", name, meta.min_args, got));
                        }
                        if got > meta.max_args {
                            return Err(format!("Z{} - Too many arguments, expected up to {}, got {}", name, meta.max_args, got));
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
            return Err("Zif - Too few arguments, expected at least 2 (condition and then branch)".to_string());
        }

        // Evaluate the condition arg to a string
        let condition_str = match self.evaluate(args[0].clone())? {
            FnOutput::Text(t) => t,
            FnOutput::Empty => String::new(),
            FnOutput::Error(e) => return Ok(FnOutput::Error(e)),
            FnOutput::UserError(e) => return Ok(FnOutput::UserError(e)),
            FnOutput::Reply => return Ok(FnOutput::Reply),
        };

        // Evaluate the condition
        let result = crate::functions::control::helpers::eval_condition(&condition_str)
            .map_err(|e| format!("Zif - {}", e))?;

        // Pick the branch to evaluate
        let branch_node = if result {
            args.into_iter().nth(1)
        } else {
            args.into_iter().nth(2)
        };

        match branch_node {
            None => Ok(FnOutput::Empty),
            Some(node) => self.evaluate(node),
        }
    }

    /// Lazy evaluation for ZtryRun{code;fallback}
    /// Evaluates code; if it produces an error, evaluates fallback instead.
    fn evaluate_try_run(&mut self, args: Vec<Node>) -> Result<FnOutput, String> {
        if args.is_empty() {
            return Err("ZtryRun - code argument is required".to_string());
        }

        // Try evaluating the first arg (the code branch)
        let result = self.evaluate(args[0].clone())?;

        match result {
            // Error in the code branch — run the fallback if provided
            FnOutput::Error(_) | FnOutput::UserError(_) => {
                match args.into_iter().nth(1) {
                    Some(fallback) => self.evaluate(fallback),
                    None => Ok(FnOutput::Empty),
                }
            }
            // Success — return the result as-is
            other => Ok(other),
        }
    }
}

