use crate::ast::Node;
use crate::context::FnMeta;
use crate::parser::parse_line;
use crate::types::{ Command, CommandOption, CommandScope, CommandType, Config, OptionType };
use std::collections::HashMap;
use std::fs;
use std::path::{ Path, PathBuf };
use std::sync::Arc;

pub struct ParseError {
    pub path: PathBuf,
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Error] {}:{}: {}", self.path.display(), self.line, self.message)
    }
}

pub struct CommandMetadata {
    pub trigger: String,
    pub name: Option<String>,
    pub description: String,
    pub command_type: CommandType,
    pub scope: CommandScope,
    pub options: Vec<CommandOption>,
}

fn report_error(err: &ParseError) {
    eprintln!("{}", err);
}

// Calculate the net change in brace depth for a line, respecting escapes and comments
fn get_brace_change(s: &str) -> i32 {
    let mut depth = 0;
    let mut chars = s.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch == '/' && depth == 0 {
            if let Some(&(_, next)) = chars.peek() {
                if next == '/' {
                    break;
                } // Rest is comment
            }
        }
        if ch == '\\' {
            if let Some(&(_, next)) = chars.peek() {
                if next == '{' || next == '}' || next == '\\' {
                    chars.next();
                    continue;
                }
            }
        }
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
        }
    }
    depth
}

fn parse_file(
    path: &Path,
    content: &str,
    registry: &HashMap<String, FnMeta>
) -> Result<(CommandMetadata, Vec<Node>), ParseError> {
    let mut trigger = None;
    let mut name = None;
    let mut description = String::from("No description provided");
    let mut command_type = CommandType::Prefix;
    let mut scope = CommandScope::Guild;
    let mut options = Vec::new();
    let mut nodes = Vec::new();

    let mut buffer = String::new();
    let mut depth = 0;

    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if depth == 0 && trimmed.starts_with('#') {
            if trimmed.starts_with("#trigger ") {
                trigger = Some(trimmed["#trigger ".len()..].trim().to_string());
            } else if trimmed.starts_with("#name ") {
                name = Some(trimmed["#name ".len()..].trim().to_string());
            } else if trimmed.starts_with("#description ") {
                description = trimmed["#description ".len()..].trim().to_string();
            } else if trimmed.starts_with("#type ") {
                command_type = match trimmed["#type ".len()..].trim() {
                    "slash" => CommandType::Slash,
                    "sub-slash" => CommandType::SubSlash,
                    "interaction" => CommandType::Interaction,
                    "event" => CommandType::Event,
                    _ => CommandType::Prefix,
                };
            } else if trimmed.starts_with("#scope ") {
                scope = match trimmed["#scope ".len()..].trim() {
                    "global" => CommandScope::Global,
                    "both" => CommandScope::Both,
                    _ => CommandScope::Guild,
                };
            } else if trimmed.starts_with("#option ") {
                let option_str = trimmed["#option ".len()..].trim();
                let parts: Vec<&str> = option_str.split('|').collect();
                if parts.len() == 4 {
                    if let Some(option_type) = OptionType::from_str(parts[2]) {
                        options.push(CommandOption {
                            name: parts[0].trim().to_string(),
                            description: parts[1].trim().to_string(),
                            option_type,
                            required: parts[3].trim() == "required",
                        });
                    } else {
                        return Err(ParseError {
                            path: path.to_path_buf(),
                            line: line_num,
                            message: format!("Unknown option type: {}", parts[2]),
                        });
                    }
                } else {
                    return Err(ParseError {
                        path: path.to_path_buf(),
                        line: line_num,
                        message: "Invalid #option format — use name|description|type|required".into(),
                    });
                }
            }
        } else {
            // Buffer the code line
            if !buffer.is_empty() {
                buffer.push('\n');
            }
            buffer.push_str(trimmed);
            depth += get_brace_change(trimmed);

            if depth == 0 {
                crate::runtime::CURRENT_LOCATION.with(|loc| {
                    *loc.borrow_mut() = (path.to_string_lossy().to_string(), line_num);
                });
                if let Some(node) = parse_line(buffer.trim(), Some(registry)) {
                    nodes.push(node);
                }
                buffer.clear();
            }
        }
    }

    if let Some(trigger) = trigger {
        Ok((
            CommandMetadata {
                trigger,
                name,
                description,
                command_type,
                scope,
                options,
            },
            nodes,
        ))
    } else {
        Err(ParseError {
            path: path.to_path_buf(),
            line: 0,
            message: "Missing #trigger".into(),
        })
    }
}

pub fn load_commands(dir: &str, registry: &HashMap<String, FnMeta>) -> HashMap<String, Command> {
    let mut commands = HashMap::new();

    let config_str = fs::read_to_string("zbr.json").unwrap_or_default();
    let config: Config = serde_json::from_str(&config_str).unwrap_or_else(|_| Config {
        status: None,
        activity: None,
        logging: true,
    });

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("No commands/ folder found");
            return commands;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("zbr") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                report_error(
                    &(ParseError {
                        path: path,
                        line: 0,
                        message: format!("Failed to read file: {}", e),
                    })
                );
                continue;
            }
        };

        match parse_file(&path, &content, registry) {
            Ok((meta, nodes)) => {
                let ast = Arc::new(Node::Concat(nodes));
                let cmd_name = meta.name.unwrap_or_else(||
                    meta.trigger.trim_start_matches(['!', '/']).to_string()
                );
                if config.logging {
                    println!("Loaded command: {} → {}", cmd_name, meta.trigger);
                }
                commands.insert(meta.trigger.clone(), Command {
                    name: cmd_name,
                    trigger: meta.trigger,
                    description: meta.description,
                    command_type: meta.command_type,
                    scope: meta.scope,
                    options: meta.options,
                    ast,
                });
            }
            Err(e) => {
                report_error(&e);
            }
        }
    }

    commands
}
