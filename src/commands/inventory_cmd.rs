use serenity::all::{Colour, CommandInteraction, Context, CreateAttachment, CreateCommand,
                    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::inventory::item::InventoryItem;
use crate::inventory::minion::MINION_BASE_MAX_SLUDGE;
use crate::nay;
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    // form the item fields
    let mut user_file = UserValues::get(&cmd.user.id);

    let inv = user_file.get_items();
    let equipped = user_file.file.inventory.equiped;

    if inv.is_empty() {
        let embed = CreateEmbed::new()
            .title("Inventory")
            .description("Your inventory is empty!")
            .color(Colour::GOLD)
            .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

        let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .embed(embed)
            .ephemeral(true));

        if let Err(err) = cmd.create_response(&ctx.http, builder).await {
            nay!("Failed to respond to command: {}", err)
        }
        return;
    }

    let mut items = Vec::new();
    for x in 0..inv.len() {
        let item = inv.get(x).unwrap();
        let x = x + 1;
        match item {
            InventoryItem::Minion(m) => {
                if m.is_full() {
                    items.push((format!("{}: Minion", x), "Full".to_string(), false));
                } else {
                    items.push((format!("{}: Minion", x),
                                format!("{}/{}", m.get_sludge_produced(), MINION_BASE_MAX_SLUDGE), true));
                }
            }
            InventoryItem::HealingPotion { health: max_effectiveness } => {
                items.push((format!("{}: Healing Potion", x), format!("{}hp", max_effectiveness), true));
            }
            InventoryItem::SuperDrill(drill) => {
                items.push((format!("{}: Super Drill", x), format!("Tier {}", drill.tier), true));
            }
            InventoryItem::SpellTome { name, damage } => {
                items.push((format!("{}: {} Tome", x, name), format!("{}-{} damage", damage.start(), damage.end()), true));
            }
            InventoryItem::Weapon { name, wtype, damage } => {
                items.push((format!("{}: {} ({})", x, name, wtype), format!("{}-{} damage", damage.start(), damage.end()), true));
            }
        }
    }

    let embed = CreateEmbed::new()
        .title("Inventory")
        .description(format!("Your items!\nEquipped: {}", if let Some(item) = equipped { item.to_string() } else { "None".to_string() }))
        .color(Colour::GOLD)
        .thumbnail("attachment://backpack.jpeg")
        .fields(items)
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true)
        .add_file(CreateAttachment::path("./images/backpack.jpeg").await.unwrap()));

    if let Err(err) = cmd.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("inventory")
        .description("View your inventory")
        .dm_permission(true)
}