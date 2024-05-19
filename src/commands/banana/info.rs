use serenity::all::{Colour, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed,
                    CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, Timestamp, UserId};
use crate::nay;
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, command: &CommandInteraction, user: &UserId) {
    let mut userfile = UserValues::get(user);

    let embed = CreateEmbed::new()
        .title(format!("{}'s Info", user.to_user(ctx).await.unwrap().global_name.unwrap()))
        .color(Colour::DARK_TEAL)
        .description("View your stats!")
        .thumbnail("attachment://george.png")
        .fields(
            vec![
                ("Bananas:", format!("{}:banana:", userfile.get_bananas()), true),
                ("Level:", format!("{}", userfile.get_level()), true),
                if userfile.get_level() < 100 {
                    ("Next Level:", format!("{}:banana: ({})", userfile.levelup_cost(), if userfile.can_levelup() { ":heavy_check_mark:" } else { ":heavy_multiplication_x:" }), true)
                } else {
                    ("Next Level:", "Prestige! :heavy_check_mark:".to_string(), true)
                },
                ("Prestige:", format!("{}", userfile.get_prestige()), true),
                if userfile.get_prestige() < 10 {
                    ("Can Prestige:", format!("{}", if userfile.can_prestige() { "Yes!" } else { "No (lvl 100)" }), true)
                } else {
                    ("Can Prestige:", "No (Ascend!)".to_string(), true)
                },
                ("Ascension:", format!("{}", userfile.get_ascension()), true),
                ("Can Ascend:", format!("{}", if userfile.can_ascend() { "Yes!" } else { "No (prestige 10 + 1mil:banana:)" }), true),
            ]
        )
        .footer(CreateEmbedFooter::new("Brought to you by GigaApe Inc©"))
        .timestamp(Timestamp::now());

    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .add_file(CreateAttachment::path("./images/george.png").await.unwrap())
        .embed(embed));

    if let Err(e) = command.create_response(&ctx.http, msg).await {
        nay!("Failed to respond to command: {}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info")
        .description("View your stats!")
        .dm_permission(true)
}