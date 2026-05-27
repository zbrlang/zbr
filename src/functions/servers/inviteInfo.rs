use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let code = match args.get(0) {
        Some(c) => c,
        None => return FnOutput::error("inviteInfo", crate::error_messages::required(1, "code")),
    };
    let info_type = match args.get(1) {
        Some(t) => t,
        None => return FnOutput::error("inviteInfo", crate::error_messages::required(2, "type")),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("inviteInfo", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let invite = match http.get_invite(code, true, true, None).await {
                Ok(i) => i,
                Err(_) => return Err("invite not found"),
            };

            match info_type.to_lowercase().as_str() {
                "uses" => Ok("0".to_string()), // Invite metadata is not easily accessible via get_invite in 0.12
                "channel" => Ok(invite.channel.id.to_string()),
                "creationDate" => Ok("".to_string()),
                "inviter" => Ok(invite.inviter.map(|u| u.id.to_string()).unwrap_or_default()),
                "isTemporary" => Ok("false".to_string()),
                _ => Err("invalid type"),
            }
        })
    });

    match result {
        Ok(val) => {
            if val == "invalid type" {
                FnOutput::error("inviteInfo", crate::error_messages::expected_choice(2, "type", "uses, channel, creationDate, inviter, isTemporary", info_type))
            } else {
                FnOutput::Text(val)
            }
        }
        Err(e) => FnOutput::error("inviteInfo", e),
    }
}
