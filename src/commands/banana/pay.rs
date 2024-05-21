use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, ResolvedOption, ResolvedValue, UserId};
use serenity::builder::CreateCommandOption;
use crate::{command_response, command_response_loud};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction, user: &UserId) {
    let Some(ResolvedOption { value: ResolvedValue::User(target, ..), .. }) = options.first() else {
        // error message
        command_response(ctx, command, "Me confused, You must bet a number of bananas").await;
        return;
    };

    let Some(ResolvedOption { value: ResolvedValue::Integer(amt), .. }) = options.last() else {
        // error message
        command_response(ctx, command, "Me confused, You must bet a number of bananas").await;
        return;
    };

    let mut userfile = UserValues::get(user);

    let amt = *amt as u64;

    // check if the user has enough to pay
    if userfile.get_bananas() < amt {
        // error message
        command_response(ctx, command, "You too poor!").await;
        return;
    }

    // remove the bananas from the user
    userfile.remove_bananas(amt);

    // add the bananas to the target
    let mut targetfile = UserValues::get(&target.id);
    targetfile.add_bananas(amt);

    // success message
    command_response_loud(ctx, command, format!("You paid {} bananas to <@{}>", amt, target.id)).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pay")
        .description("Pay someone bananas")
        .add_option(CreateCommandOption::new(CommandOptionType::Mentionable, "target", "the user to pay")
            .required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::Integer, "amount", "the amount to pay the user")
            .required(true))
        .dm_permission(false)
}
