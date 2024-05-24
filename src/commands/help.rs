use serenity::all::{Colour, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, Permissions};
use serenity::builder::CreateEmbed;
use crate::nay;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let mut embed = CreateEmbed::new()
        .title("Help")
        .description("Me can do many things!")
        .color(Colour::GOLD)
        .thumbnail("attachment://george.png")
        .fields(
            vec![
                ("/help", "Me show you commands", true),
                ("/info", "View your stats", true),
                ("/inventory", "View your inventory", true),
                ("/discard", "Discard an item", true),
                ("/shop", "View the shop", true),
                ("/buy", "Buy an item from the shop", true),
                ("/leaderboard", "See no-lifes", true),
                ("/levelup", "Buy level with banana", true),
                ("/prestige", "Prestige at level 100", true),
                ("/ascend", "Show you have no life at prestige 10", true),
                ("/mine", "Work for bananas and more", true),
                ("/pay", "Give banana", true),
                ("/collect_minions", "Collect sludge mined by your minions", true),

                ("/blackjack", "Gamble bananas in a game of blackjack", true),
                ("/fiftyfifty", "Gamble bananas with a 50% chance", true),
                ("/slots", "You spin me right round", true),
            ]
        )
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

    // if the sender is an admin
    if let Some(member) = cmd.member.as_ref() {
        if let Some(perms) = member.permissions {
            if perms.contains(Permissions::ADMINISTRATOR) {
                embed = embed.field("/admin_channel", "Add and remove channels from Ape Bot's allowed channels (allowed commands)", true);
            }
        }
    }

    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true)
        .add_file(CreateAttachment::path("./images/george.png").await.unwrap()));

    if let Err(err) = cmd.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help")
        .description("List available commands")
        .dm_permission(true)
}