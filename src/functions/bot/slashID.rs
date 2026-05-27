use crate::context::{DiscordContext, FnOutput};
use crate::types::CommandType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned();
    
    let query_name = match name {
        Some(n) => n,
        None => {
            let cmds = crate::loader::load_commands("commands");
            let mut is_slash = false;
            if let Some(trigger) = &ctx.trigger {
                if let Some(cmd) = cmds.get(trigger) {
                    if matches!(cmd.command_type, CommandType::Slash) {
                        is_slash = true;
                    }
                }
            }
            if !is_slash {
                return FnOutput::error("slashID", crate::error_messages::requires_set_first("command type to slash"));
            }
            ctx.command_name.clone()
        }
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::Text(String::new()),
    };

    let guild_id = ctx.guild_id.parse::<u64>().ok();

    let id = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            if let Some(gid) = guild_id {
                if let Ok(cmds) = http.get_guild_commands(gid.into()).await {
                    for cmd in cmds {
                        if cmd.name == query_name {
                            return Some(cmd.id.to_string());
                        }
                    }
                }
            }
            if let Ok(cmds) = http.get_global_commands().await {
                for cmd in cmds {
                    if cmd.name == query_name {
                        return Some(cmd.id.to_string());
                    }
                }
            }
            None
        })
    });

    match id {
        Some(i) => FnOutput::Text(i),
        None => FnOutput::Text(String::new())
    }
}
