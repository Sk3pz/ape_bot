use serenity::all::{CommandInteraction, Context, CreateCommand};
use crate::command_response;
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let mut user_file = UserValues::get(&cmd.user.id);

    // get the user's minions
    let minions = user_file.get_minions();

    if minions.is_empty() {
        command_response(&ctx, &cmd, "You don't have any minions to collect sludge from.").await;
        return;
    }

    let total_sludge: u64 = user_file.collect_minions();
    let bananas = total_sludge * 250;

    user_file.add_bananas(bananas);

    command_response(&ctx, &cmd, format!("You collected {} sludge from your minions totalling {}:banana:", total_sludge, bananas)).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("collect_minions")
        .description("Collect the sludge mined by your minions")
        .dm_permission(true)
}