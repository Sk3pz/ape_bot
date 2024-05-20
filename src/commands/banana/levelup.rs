use serenity::all::{Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, UserId};
use serenity::builder::CreateEmbedFooter;
use serenity::model::Timestamp;
use crate::{command_response, middle_finger_image, nay};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction, user: &UserId) {
    let mut userfile = UserValues::get(user);

    // check if the user has enough bananas
    if !userfile.can_levelup() {
        let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .content("No.")
            .add_file(CreateAttachment::path(middle_finger_image().await).await.unwrap()));
        if let Err(e) = command.create_response(&ctx.http, msg).await {
            nay!("Failed to respond to command: {}", e);
        }
        return;
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(opt), .. }) = options.first() {
        let opt = opt.to_string();
        match opt.as_str() {
            "all" => {
                while userfile.can_levelup() {
                    userfile.levelup()
                }
            }
            "one" => {
                userfile.levelup()
            }
            _ => {
                command_response(ctx, command, "That not valid option! either `one` or `all`!").await;
                return;
            }
        }
    } else {
        userfile.levelup()
    }

    // show levelup embed
    let embed = CreateEmbed::new()
        .title("Level Up!")
        .description(format!("Congratulations, {}!", user.to_user(ctx).await.unwrap().global_name.unwrap()))
        .thumbnail("attachment://george.png")
        .color(Colour::DARK_GOLD)
        .fields(
            vec![
                ("Your Level:", format!("{}", userfile.get_level()), true),
                ("Your Bananas:", format!("{}:banana:", userfile.get_bananas()), true),
                if userfile.get_level() < 100 {
                    ("Next Level:", format!("{}:banana: ({})", userfile.levelup_cost(), if userfile.can_levelup() { ":heavy_check_mark:" } else { ":heavy_multiplication_x:" }), true)
                } else {
                    ("Next Level:", "Prestige! :heavy_check_mark:".to_string(), true)
                },
                if userfile.get_prestige() < 10 {
                    ("Can Prestige:", format!("{}", if userfile.can_prestige() { "Yes!" } else { "No (lvl 100)" }), true)
                } else {
                    ("Can Prestige:", "No (Ascend!)".to_string(), true)
                },
                ("Can Ascend:", format!("{}", if userfile.can_ascend() { "Yes!" } else { "No (prestige 10 + 1mil:banana:)" }), true),
            ]
        )
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        .timestamp(Timestamp::now());

    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .add_file(CreateAttachment::path("./images/george.png").await.unwrap()));

    if let Err(e) = command.create_response(&ctx.http, msg).await {
        nay!("Failed to respond to command: {}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("levelup")
        .description("Level up!")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "amount", "`all` or `one`")
            .required(false))
        .dm_permission(true)
}