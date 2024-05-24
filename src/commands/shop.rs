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
            // todo: make super pickaxe upgradable rather than a one time purchase
            ("1: Super Pickaxe", if user_file.has_super_drill() { "You already own this item!" } else { "50:zap:" }, false),
            ("2: Minion", "10:zap:", false),
            ("3: Healing Potion (5hp)", "2:zap:", false),
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