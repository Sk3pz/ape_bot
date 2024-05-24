use serenity::all::{Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue};
use crate::{command_response, nay};
use crate::inventory::item::InventoryItem;
use crate::inventory::super_drill::SuperDrill;
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, cmd: &CommandInteraction) {
    let Some(ResolvedOption { value: ResolvedValue::Integer(item, ..), .. }) = options.first() else {
        // error message
        command_response(ctx, &cmd, "Me confused, Enter a number for the mine level you want to mine at.").await;
        return;
    };

    // form the item fields
    let mut user_file = UserValues::get(&cmd.user.id);

    // if the user's inventory is full
    if user_file.file.inventory.is_full() {
        command_response(ctx, &cmd, "Your inventory is full! (use `/discard #` to throw out an item!").await;
        return;
    }

    let embed = match item {
        1 => { // super pickaxe
            if user_file.has_super_drill() {
                command_response(ctx, &cmd, "You already own this item!").await;
                return;
            }
            if user_file.get_super_nanners() < 50 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::SuperDrill(SuperDrill { tier: 1 }));
            user_file.remove_super_nanners(50);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased the Super Pickaxe!")
                .color(Colour::GOLD)
                .field("Cost", "50:zap", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        2 => { // minion
            if user_file.has_super_drill() {
                command_response(ctx, &cmd, "You already own this item!").await;
                return;
            }
            if user_file.get_super_nanners() < 10 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::SuperDrill(SuperDrill { tier: 1 }));
            user_file.remove_super_nanners(10);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Minion!")
                .color(Colour::GOLD)
                .field("Cost", "10:zap", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        3 => { // healing potion
            if user_file.has_super_drill() {
                command_response(ctx, &cmd, "You already own this item!").await;
                return;
            }
            if user_file.get_super_nanners() < 2 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::HealingPotion { health: 5 });
            user_file.remove_super_nanners(2);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Healing Potion!")
                .color(Colour::GOLD)
                .field("Cost", "2:zap", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        _ => {
            command_response(ctx, &cmd, "Invalid item number!").await;
            return;
        }
    };

    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true)
        .add_file(CreateAttachment::path("./images/shop.jpeg").await.unwrap()));

    if let Err(err) = cmd.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("buy")
        .description("Buy an item from the shop")
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "item", "The item you wish to buy (see /shop)")
            .required(true))
        .dm_permission(true)
}