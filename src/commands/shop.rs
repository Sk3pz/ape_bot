use serenity::all::{Colour, CommandInteraction, Context, CreateAttachment, CreateCommand,
                    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::nay;
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    // form the item fields
    let mut user_file = UserValues::get(&cmd.user.id);

    let embed = CreateEmbed::new()
        .title("Shop")
        .description("See items you can buy!\nPurchase items with `/buy #`\n:zap: = super nanners")
        .color(Colour::GOLD)
        .thumbnail("attachment://shop.jpeg")
        .fields(vec![
            ("Your Super Nanners:", format!("{}:zap:", user_file.get_super_nanners()), false),
            // todo: make super drill upgradable rather than a one time purchase
            ("1: Super Drill", if user_file.has_super_drill() { "You already own this item!" } else { "50:zap:" }.to_string(), false),
            ("2: Minion", "30:zap:".to_string(), false),
            ("3: Healing Potion (25hp)", "2:zap:".to_string(), false),
        ])
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true)
        .add_file(CreateAttachment::path("./images/shop.jpeg").await.unwrap()));

    if let Err(err) = cmd.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("shop")
        .description("View the shop!")
        .dm_permission(true)
}