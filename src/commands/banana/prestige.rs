use serenity::all::{Colour, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, UserId};
use serenity::builder::CreateEmbedFooter;
use serenity::model::Timestamp;
use crate::{command_response, nay};
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, command: &CommandInteraction, user: &UserId) {
    let mut userfile = UserValues::get(user);

    // check if the user has enough bananas
    if !userfile.can_prestige() {
        command_response(ctx, command, "You must be level 100 to prestige!").await;
        return;
    }

    userfile.prestige();
    // show levelup embed
    let embed = CreateEmbed::new()
        .title("PRESTIGE! :tada: :tada: :tada:")
        .description(format!("Congratulations, {}!", user.to_user(ctx).await.unwrap().global_name.unwrap()))
        .thumbnail("attachment://monkey.png")
        .color(Colour::DARK_GOLD)
        .fields(
            vec![
                ("Prestige:", format!("{}", userfile.get_prestige()), true),
                ("Level:", format!("{}:banana:", userfile.get_level()), true),
                ("Next Level:", format!("{}:banana:", userfile.levelup_cost()), true),
                ("Bananas:", format!("{}:banana:", userfile.get_bananas()), true),
                ("Ascension:", format!("{}", userfile.get_ascension()), true),
            ]
        )
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        .timestamp(Timestamp::now());

    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .add_file(CreateAttachment::path("./images/monkey.png").await.unwrap()));

    if let Err(e) = command.create_response(&ctx.http, msg).await {
        nay!("Failed to respond to command: {}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("prestige")
        .description("Prestige!")
        .dm_permission(true)
}