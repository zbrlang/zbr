use crate::context::{DiscordContext, FnOutput, ModalData};

/// ZnewModal{modalID;title}
/// Creates a modal. Only valid inside an onInteraction handler.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let modal_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("newModal", "modalID is required"),
    };
    let title = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("newModal", "title is required"),
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.components.lock().await.modal = Some(ModalData {
                modal_id,
                title,
                fields: Vec::new(),
            });
        })
    });

    FnOutput::Empty
}
