use std::sync::Arc;
use rand::{Rng, thread_rng};
use serenity::all::{ChannelId, Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedFooter, CreateMessage, Http, Mentionable, UserId};
use crate::{command_response, MINING, nay, SLUDGE_BANANA_WORTH};
use crate::userfile::UserValues;

const BASE_MINE_TIME: u64 = 60;

pub async fn finish_mine(http: Arc<Http>, channel: ChannelId, sender: UserId) {
    // remove the user mining
    MINING.lock().unwrap().retain(|x| x != &sender);

    let mut user_file = UserValues::get(&sender);
    // sludge monster event (1/100 chance) TODO
    if thread_rng().gen_range(0..100) == 0 {
        // COMING SOON
        // let embed = CreateEmbed::new()
        //     .title("SLUDGE MONSTER EVENT")
        //     .description("A sludge monster has appeared! You must defeat it to continue mining.")
        //     .color(Colour::DARK_GREEN)
        //     .timestamp(Timestamp::now());

        // let builder = CreateMessage::new()
        //     .content(format!("{}", sender.mention()))
        //     .embed(embed);
        //
        // if let Err(e) = channel.send_message(&http, builder).await {
        //     nay!("Failed to send message: {}", e);
        // }

        // return;
    }

    // determine the amount of sludge
    let sludge = thread_rng().gen_range(0..=10);

    if sludge == 0 {
        let embed = CreateEmbed::new()
            .title("MINING FAILURE")
            .description("A.P.E. Inc© is disappointed in you. Do better next time.".to_string())
            .field("Sludge Mined:", "0", true)
            .field("Bananas Earned:", "0", true)
            .color(Colour::RED)
            .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

        let builder = CreateMessage::new()
            .content(format!("{}", sender.mention()))
            .embed(embed);

        if let Err(e) = channel.send_message(&http, builder).await {
            nay!("Failed to send message: {}", e);
        }

        return;
    }

    // determine the value of the mined sludge
    let value = sludge * SLUDGE_BANANA_WORTH;

    // update the user file
    user_file.add_bananas(value);

    // send the success message
    let embed = CreateEmbed::new()
        .title("MINING SUCCESS")
        .description("A.P.E. Inc© is proud of your work".to_string())
        .field("Sludge Mined:", format!("{}", sludge), true)
        .field("Bananas Earned:", format!("{}:banana:", value), true)
        .color(Colour::DARK_GREEN)
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

    let builder = CreateMessage::new()
        .content(format!("{}", sender.mention()))
        .embed(embed);

    if let Err(e) = channel.send_message(&http, builder).await {
        nay!("Failed to send message: {}", e);
    }
}

pub async fn run(ctx: &Context, channel: &ChannelId, command: &CommandInteraction, sender: &UserId) {
    if MINING.lock().unwrap().contains(&sender) {
        command_response(ctx, command, "You are already mining!").await;
        return;
    }

    let mut user_file = UserValues::get(&sender);

    // add user to mining
    MINING.lock().unwrap().push(sender.clone());

    // determine the time
    let time = if user_file.has_super_drill() {
        ((BASE_MINE_TIME as f32) / (1.5 * user_file.get_ascension() as f32)).min(3.0) as u64
    } else {
        BASE_MINE_TIME
    };

    let http = ctx.http.clone();
    let channel_id = *channel;
    let sender = sender.clone();

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(time)).await;
        finish_mine(http, channel_id, sender).await;
    });

    // send a message that they are mining
    command_response(ctx, command, "You have begun mining...").await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("mine")
        .description("Mine for resources")
        .dm_permission(false)
}