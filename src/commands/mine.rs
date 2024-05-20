use serenity::all::{ChannelId, CommandInteraction, Context, CreateCommand, UserId};
use crate::{command_response, MINING};
use crate::userfile::UserValues;

pub async fn finish_mine(ctx: &Context, channel: &ChannelId, sender: &UserId, user_file: UserValues) {
    todo!()
}

pub async fn run(ctx: &Context, channel: &ChannelId, command: &CommandInteraction, sender: &UserId) {
    if MINING.lock().unwrap().contains(&sender) {
        command_response(ctx, command, "You are already mining!").await;
        return;
    }

    let mut user_file = UserValues::get(&sender);

    // add user to mining
    MINING.lock().unwrap().push(sender.clone());

}

pub fn register() -> CreateCommand {
    CreateCommand::new("mine")
        .description("Mine for resources")
        .dm_permission(false)
}