use serenity::all::{Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, Timestamp};
use crate::{command_response, nay};
use crate::inventory::item::{InventoryItem, WeaponType};
use crate::inventory::minion::Minion;
use crate::inventory::super_drill::SuperDrill;
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, cmd: &CommandInteraction) {
    let Some(ResolvedOption { value: ResolvedValue::Integer(item, ..), .. }) = options.first() else {
        // error message
        command_response(ctx, &cmd, "Me confused, Enter the number of the item you wish to buy").await;
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
        1 => { // Super Pickaxe
            if user_file.has_super_drill() {
                command_response(ctx, &cmd, "You already own this item!").await;
                return;
            }
            if user_file.get_super_nanners() < 15 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::SuperDrill(SuperDrill { tier: 1 }));
            user_file.remove_super_nanners(15);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased the Super Pickaxe!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "15:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        2 => { // Minion
            if user_file.get_super_nanners() < 10 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::Minion(Minion { level: 1, mining_start: Timestamp::now() }));
            user_file.remove_super_nanners(10);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Minion!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "10:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        3 => { // Stick of Thwacking
            if user_file.get_super_nanners() < 5 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::Weapon {
                name: "Stick of Thwacking".to_string(),
                wtype: WeaponType::Stick,
                damage: 5..=15,
            });
            user_file.remove_super_nanners(5);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Stick of Thwacking!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "5:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        4 => { // Excalibur (Sword)
            if user_file.get_super_nanners() < 30 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::Weapon {
                name: "Excalibur".to_string(),
                wtype: WeaponType::Sword,
                damage: 50..=100,
            });
            user_file.remove_super_nanners(30);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased Excalibur!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "30:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        5 => { // Spell Tome - Fireball 25-40hp
            if user_file.get_super_nanners() < 5 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::SpellTome { name: "Fireball".to_string(), damage: 25..=40 });
            user_file.remove_super_nanners(3);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Fireball Spell Tome!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "5:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        6 => { // Spell Tome - Mighty Winds 5-25hp
            if user_file.get_super_nanners() < 3 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::SpellTome { name: "Mighty Winds".to_string(), damage: 5..=25 });
            user_file.remove_super_nanners(3);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Mighty Winds Spell Tome!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "3:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        7 => { // Healing Potion - 10hp
            if user_file.get_super_nanners() < 1 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::HealingPotion { health: 10 });
            user_file.remove_super_nanners(1);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Healing Potion (10hp)!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "1:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        8 => { // Healing Potion - 25hp
            if user_file.get_super_nanners() < 2 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::HealingPotion { health: 25 });
            user_file.remove_super_nanners(2);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased a Healing Potion (25hp)!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "2:zap:", true)
                .field("Balance", format!("{}:zap:", user_file.get_super_nanners()), true)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        }
        27 => { // special weapon - The Throngler for 1:zap:
            if user_file.get_super_nanners() < 1 {
                command_response(ctx, &cmd, "You don't have enough super nanners!").await;
                return;
            }
            user_file.add_item(InventoryItem::Weapon {
                name: "The Throngler".to_string(),
                wtype: WeaponType::Sword,
                damage: 25..=30,
            });
            user_file.remove_super_nanners(15);

            CreateEmbed::new()
                .title("Purchase Successful")
                .description("You have purchased The Throngler!")
                .thumbnail("attachment://shop.jpeg")
                .color(Colour::GOLD)
                .field("Cost", "15:zap:", true)
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