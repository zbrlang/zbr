use crate::ast::Node;
use crate::parser::parse_line;
use crate::types::{Command, CommandOption, CommandScope, CommandType, Config, OptionType};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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

fn parse_file(path: &Path, content: &str) -> Result<(CommandMetadata, Vec<Node>), ParseError> {
    let mut trigger = None;
    let mut name = None;
    let mut description = String::from("No description provided");
    let mut command_type = CommandType::Prefix;
    let mut scope = CommandScope::Guild;
    let mut options = Vec::new();
    let mut nodes = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with("#trigger ") {
            trigger = Some(line["#trigger ".len()..].trim().to_string());
        } else if line.starts_with("#name ") {
            name = Some(line["#name ".len()..].trim().to_string());
        } else if line.starts_with("#description ") {
            description = line["#description ".len()..].trim().to_string();
        } else if line.starts_with("#type ") {
            command_type = match line["#type ".len()..].trim() {
                "slash" => CommandType::Slash,
                "sub-slash" => CommandType::SubSlash,
                "interaction" => CommandType::Interaction,
                "event" => CommandType::Event,
                _ => CommandType::Prefix,
            };
        } else if line.starts_with("#scope ") {
            scope = match line["#scope ".len()..].trim() {
                "global" => CommandScope::Global,
                "both" => CommandScope::Both,
                _ => CommandScope::Guild,
            };
        } else if line.starts_with("#option ") {
            let option_str = line["#option ".len()..].trim();
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
        } else if let Some(node) = parse_line(line) {
            nodes.push(node);
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

pub fn load_commands(dir: &str) -> HashMap<String, Command> {
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
                report_error(&ParseError {
                    path: path,
                    line: 0,
                    message: format!("Failed to read file: {}", e),
                });
                continue;
            }
        };

        match parse_file(&path, &content) {
            Ok((meta, nodes)) => {
                let ast = Arc::new(Node::Concat(nodes));
                let cmd_name = meta.name.unwrap_or_else(|| meta.trigger.trim_start_matches(['!', '/']).to_string());
                if config.logging {
                    println!("Loaded command: {} → {}", cmd_name, meta.trigger);
                }
                commands.insert(
                    meta.trigger.clone(),
                    Command {
                        name: cmd_name,
                        trigger: meta.trigger,
                        description: meta.description,
                        command_type: meta.command_type,
                        scope: meta.scope,
                        options: meta.options,
                        ast,
                    },
                );
            }
            Err(e) => {
                report_error(&e);
            }
        }
    }

    commands
}
