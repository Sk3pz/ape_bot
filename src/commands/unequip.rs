use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption};
use crate::command_response;
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    // get the userfile
    let mut user_file = UserValues::get(&cmd.user.id);

    // check if the user has an equipped item
    if user_file.get_equiped().is_none() {
        command_response(ctx, &cmd, "You don't have anything equipped!").await;
        return;
    }

    // unequip the item
    user_file.unequip_item();

    // respond
    command_response(ctx, &cmd, "Item unequipped!").await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("unequip")
        .description("Unequip an item")
        .dm_permission(true)
}