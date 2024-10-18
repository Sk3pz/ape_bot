use serenity::all::{Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue};
use crate::{command_response, nay};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, cmd: &CommandInteraction) {
    let Some(ResolvedOption { value: ResolvedValue::Integer(item, ..), .. }) = options.first() else {
        // error message
        command_response(ctx, &cmd, "Me confused, Enter the number of the item you wish to equip from your inventory.").await;
        return;
    };

    // get the userfile
    let mut user_file = UserValues::get(&cmd.user.id);

    // get the accurate item slot in the inventory
    let slot = (item - 1) as u32;

    // ensure the slot is valid
    if slot < 0 || slot >= user_file.get_items().len() as u32 {
        command_response(ctx, &cmd, "Invalid item slot!").await;
        return;
    }

    // equip the item
    let equiped = user_file.equip_item(slot);

    // get the item
    let eitem = user_file.get_equiped().unwrap();

    let embed = if equiped {
        CreateEmbed::new()
            .title("Item Equipped!")
            .description(format!("You have equipped your {}!", eitem).as_str())
            .thumbnail("attachment://backpack.jpeg")
            .color(Colour::GOLD)
            .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
    } else {
        command_response(ctx, &cmd, "You can't equip that!").await;
        return;
    };

    let builder = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true)
        .add_file(CreateAttachment::path("./images/backpack.jpeg").await.unwrap()));

    if let Err(err) = cmd.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("equip")
        .description("Equip an item from your inventory.")
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                "item", "The slot number of the item you wish to equip")
        .required(true))
        .dm_permission(true)
}