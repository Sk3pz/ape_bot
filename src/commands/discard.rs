use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
                    ResolvedOption, ResolvedValue};
use crate::{command_response};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, cmd: &CommandInteraction) {
    let Some(ResolvedOption { value: ResolvedValue::Integer(item, ..), .. }) = options.first() else {
        // error message
        command_response(ctx, &cmd, "Me confused, Enter a number for the mine level you want to mine at.").await;
        return;
    };

    if *item < 0 {
        command_response(ctx, &cmd, "Invalid item number! (must be positive)").await;
        return;
    }

    let item = *item as usize;

    let mut user_file = UserValues::get(&cmd.user.id);

    // ensure the number is valid
    if item >= user_file.get_items().len() {
        command_response(ctx, &cmd, "Invalid item number!").await;
        return;
    }

    let item_clone = user_file.get_items()[item].clone();

    user_file.remove_item_index(item);

    command_response(ctx, &cmd, format!("{} discarded!", item_clone)).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("discard")
        .description("Discard an item from your inventory!")
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "item", "The item you wish to discard (see /inventory)")
            .required(true))
        .dm_permission(true)
}