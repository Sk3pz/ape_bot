use serenity::all::{CommandInteraction, Context, CreateAttachment, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::{middle_finger_image, nay};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .content("No.")
        .add_file(CreateAttachment::path(middle_finger_image().await).await.unwrap()));
    if let Err(e) = cmd.create_response(&ctx.http, msg).await {
        nay!("Failed to respond to command: {}", e);
    }
    return;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help")
        .description("List available commands")
        .dm_permission(false)
}