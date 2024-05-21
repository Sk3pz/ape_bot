use std::sync::Arc;
use rand::{Rng, thread_rng};
use serenity::all::{ChannelId, Colour, CommandInteraction, Context, CreateAttachment, CreateCommand,
                    CreateEmbed, CreateEmbedFooter, CreateMessage, Http, Mentionable, Timestamp, UserId};
use crate::{command_response, GAMES, MINING, nay, SLUDGE_BANANA_WORTH};
use crate::games::{GameHandler, Games};
use crate::games::sludge_monster_battle::SludgeMonsterBattle;
use crate::userfile::UserValues;

const BASE_MINE_TIME: u64 = 60;

pub async fn finish_mine(http: Arc<Http>, channel: ChannelId, sender: UserId) {
    // remove the user mining
    MINING.lock().unwrap().retain(|x| x != &sender);

    let mut user_file = UserValues::get(&sender);
    // sludge monster event (1/100 chance)
    if thread_rng().gen_range(0..8) == 0 {
        let battle = SludgeMonsterBattle::new();
        let thumbnail = battle.thumbnail.clone();

        let embed = CreateEmbed::new()
            .title("SLUDGE MONSTER EVENT")
            .thumbnail(format!("attachment://{}", thumbnail))
            .description("A sludge monster has appeared! You must defeat it to continue mining.")
            .fields(
                vec![
                    ("Boss Health", format!("{}", battle.boss_health), true),
                    ("Your Health", "100".to_string(), true),
                    ("Options: ", "`attack` `run` or `surrender`".to_string(), false),
                ]
            )
            .color(Colour::DARK_GREEN)
            .timestamp(Timestamp::now());

        // create the game
        let game = GameHandler::new(sender.clone(), Games::SludgeMonsterBattle(battle));

        let builder = CreateMessage::new()
            .content(format!("{}", sender.mention()))
            .embed(embed)
            .add_file(CreateAttachment::path(format!("./images/sludge_monsters/{}", thumbnail)).await.unwrap());

        if let Err(e) = channel.send_message(&http, builder).await {
            nay!("Failed to send message: {}", e);
        }

        // add the game to GAMES
        GAMES.lock().unwrap().insert(game);

        return;
    }

    // determine the amount of sludge
    let sludge = thread_rng().gen_range(0..=10);

    if sludge == 0 {
        let embed = CreateEmbed::new()
            .title("MINING FAILURE")
            .thumbnail("attachment://disappointed.jpeg")
            .description("A.P.E. Inc© is disappointed in you. Do better next time.".to_string())
            .field("Sludge Mined:", "0", true)
            .field("Bananas Earned:", "0", true)
            .color(Colour::RED)
            .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

        let builder = CreateMessage::new()
            .content(format!("{}", sender.mention()))
            .embed(embed)
            .add_file(CreateAttachment::path("./images/disappointed.jpeg").await.unwrap());

        if let Err(e) = channel.send_message(&http, builder).await {
            nay!("Failed to send message: {}", e);
        }

        // if let Err(e) = command.edit_response(http, EditInteractionResponse::new()
        //     .content(format!("{}", sender.mention()))
        //     .embed(embed)).await {
        //     nay!("Failed to send message: {}", e);
        // }

        return;
    }

    // determine the value of the mined sludge
    let value = sludge * SLUDGE_BANANA_WORTH;

    // update the user file
    user_file.add_bananas(value);

    // send the success message
    let embed = CreateEmbed::new()
        .title("MINING SUCCESS")
        .thumbnail("attachment://mining_ape.jpeg")
        .description("A.P.E. Inc© is proud of your work".to_string())
        .field("Sludge Mined:", format!("{}", sludge), true)
        .field("Bananas Earned:", format!("{}:banana:", value), true)
        .color(Colour::DARK_GREEN)
        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

    let builder = CreateMessage::new()
        .content(format!("{}", sender.mention()))
        .embed(embed)
        .add_file(CreateAttachment::path("./images/mining_ape.jpeg").await.unwrap());

    if let Err(e) = channel.send_message(&http, builder).await {
        nay!("Failed to send message: {}", e);
    }

    // if let Err(e) = command.edit_response(http, EditInteractionResponse::new()
    //     .content(format!("{}", sender.mention()))
    //     .embed(embed)).await {
    //     nay!("Failed to send message: {}", e);
    // }
}

pub async fn run(ctx: &Context, channel: &ChannelId, command: CommandInteraction, sender: &UserId) {
    if MINING.lock().unwrap().contains(&sender) {
        command_response(ctx, &command, "You are already mining!").await;
        return;
    }

    let mut in_game = false;
    // if the user is battling a sludge monster
    if let Some(_game) = GAMES.lock().unwrap().get_player_game_instance(sender) {
        // if let Games::SludgeMonsterBattle(_) = game.game {
        //     in_game = true;
        // }
        in_game = true;
    }

    if in_game {
        //command_response(ctx, &command, "You are currently battling a sludge monster!").await;
        command_response(ctx, &command, "Finish your current game first!").await;
        return;
    }

    let mut user_file = UserValues::get(&sender);

    // add user to mining
    MINING.lock().unwrap().push(sender.clone());

    // determine the time
    let time = if user_file.has_super_drill() {
        ((BASE_MINE_TIME as f32) / (2.0 * (user_file.get_prestige() + 1) as f32)).min(4.0) as u64
    } else {
        BASE_MINE_TIME
    };

    let http = ctx.http.clone();
    let channel_id = *channel;
    let sender = sender.clone();

    // command.defer_ephemeral(&ctx.http).await.unwrap();
    //send a message that they are mining
    command_response(ctx, &command, "You have begun mining...").await;

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(time)).await;
        finish_mine(http, channel_id, sender).await;
    });
}

pub fn register() -> CreateCommand {
    CreateCommand::new("mine")
        .description("Mine for resources")
        .dm_permission(false)
}