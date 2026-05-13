use crate::types::{Command, CommandOption, CommandScope, CommandType, OptionType};
use std::collections::HashMap;
use std::fs;

pub fn load_commands(dir: &str) -> HashMap<String, Command> {
    let mut commands = HashMap::new();

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
            Err(_) => continue,
        };

        let mut trigger = None;
        let mut name = None;
        let mut description = String::from("No description provided");
        let mut command_type = CommandType::Prefix;
        let mut scope = CommandScope::Guild;
        let mut options = Vec::new();
        let mut code_lines = Vec::new();

        for line in content.lines() {
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
                        eprintln!("Unknown option type: {}", parts[2]);
                    }
                } else {
                    eprintln!(
                        "Invalid #option format in {:?} — use name|description|type|required",
                        path
                    );
                }
            } else {
                code_lines.push(line);
            }
        }

        if let Some(t) = trigger {
            let code = code_lines.join("\n");
            let cmd_name = name.unwrap_or_else(|| t.trim_start_matches(['!', '/']).to_string());
            println!("Loaded command: {} → {}", cmd_name, t);
            commands.insert(
                t.clone(),
                Command {
                    name: cmd_name,
                    trigger: t,
                    description,
                    command_type,
                    scope,
                    options,
                    code,
                },
            );
        } else {
            eprintln!("Skipping {:?} — no #trigger found", path);
        }
    }

    commands
}
