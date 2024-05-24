use std::sync::Arc;
use rand::{Rng, thread_rng};
use serenity::all::{ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateMessage, Http, Mentionable, ResolvedOption, ResolvedValue, Timestamp, UserId};
use crate::{command_response, GAMES, MINING, nay};
use crate::games::{GameHandler, Games};
use crate::games::mine_battle::MineBattle;
use crate::mine_data::Mine;
use crate::userfile::UserValues;

const BASE_MINE_TIME: u64 = 60;

pub async fn finish_mine(http: Arc<Http>, channel: ChannelId, sender: UserId) {
    // remove the user mining
    MINING.lock().unwrap().retain(|x| x != &sender);

    let mut user_file = UserValues::get(&sender);

    let mine = Mine::get();

    let current_tier = mine.get_tier(user_file.get_mine_tier());

    // check that the user has a super drill if it is required
    if current_tier.required_super_drill_tier >= user_file.get_super_drill_tier() {
        let embed = CreateEmbed::new()
            .title("MINING FAILURE")
            .thumbnail("attachment://disappointed.jpeg")
            .description("You need a better drill to mine here.".to_string())
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

        return;
    }

    // creature (12.5% chance)
    if thread_rng().gen_range(0..8) == 0 {
        let creature = current_tier.random_enemy();

        let battle = MineBattle::new(creature.clone(), current_tier.sludge_worth);
        let thumbnail = creature.thumbnail.clone();

        let embed = CreateEmbed::new()
            .title("CREATURE EVENT")
            .thumbnail(format!("attachment://{}", thumbnail))
            .description("A sludge monster has appeared! You must defeat it to continue mining.")
            .fields(
                vec![
                    ("Creature Health", format!("{}", battle.enemy_health), true),
                    ("Your Health", "100".to_string(), true),
                    ("Options: ", "`attack`, `run`, `item {inventory slot #}` or `surrender`".to_string(), false),
                ]
            )
            .color(Colour::DARK_GREEN)
            .timestamp(Timestamp::now());

        // create the game
        let game = GameHandler::new(sender.clone(), Games::MineBattle(battle));

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

    // super nanner chance
    let super_nanner_chance = thread_rng().gen_range(0.0..1.0);
    if current_tier.super_nanner_chance <= super_nanner_chance {
        user_file.add_super_nanners(1);
        let embed = CreateEmbed::new()
            .title("SUPER NANNER EVENT")
            .thumbnail("attachment://super_nanner.jpeg")
            .description("You have found a super nanner!".to_string())
            .color(Colour::DARK_GREEN)
            .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"));

        let builder = CreateMessage::new()
            .content(format!("{}", sender.mention()))
            .embed(embed)
            .add_file(CreateAttachment::path("./images/super_nanner.jpeg").await.unwrap());

        if let Err(e) = channel.send_message(&http, builder).await {
            nay!("Failed to send message: {}", e);
        }

        return;
    }

    // item drop chance
    let item_drop_chance = thread_rng().gen_range(0.0..1.0);
    if item_drop_chance <= 0.2 {
        let item = current_tier.drop_table.random_item();

        let embed = if user_file.file.inventory.is_full() {
            CreateEmbed::new()
                .title("ITEM DROP")
                .description("You have found an item, but your inventory is too full!".to_string())
                .field("Item:", item.to_string(), true)
                .color(Colour::RED)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        } else {
            user_file.add_item(item.clone());

            CreateEmbed::new()
                .title("ITEM DROP")
                .description("You have found an item!".to_string())
                .field("Item:", item.to_string(), true)
                .color(Colour::DARK_GREEN)
                .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
        };

        let builder = CreateMessage::new()
            .content(format!("{}", sender.mention()))
            .embed(embed);

        if let Err(e) = channel.send_message(&http, builder).await {
            nay!("Failed to send message: {}", e);
        }

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
    let value = sludge * current_tier.sludge_worth;

    // update the user file
    user_file.add_bananas(value as u64);

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

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, channel: &ChannelId, command: CommandInteraction, sender: &UserId) {
    if MINING.lock().unwrap().contains(&sender) {
        command_response(ctx, &command, "You are already mining!").await;
        return;
    }

    let mine = Mine::get();

    let mut in_game = false;
    // if the user is playing a game
    if let Some(_game) = GAMES.lock().unwrap().get_player_game_instance(sender) {
        in_game = true;
    }

    if in_game {
        //command_response(ctx, &command, "You are currently battling a sludge monster!").await;
        command_response(ctx, &command, "Finish your current game first!").await;
        return;
    }

    let mut user_file = UserValues::get(&sender);

    // set the user's tier
    if let Some(ResolvedOption { value: ResolvedValue::Integer(tier, ..), .. }) = options.first() {
        let tier = *tier;
        // ensure tier is a valid tier
        if tier < 0 || tier >= mine.tiers.len() as i64 {
            command_response(ctx, &command, "Invalid mine tier!").await;
            return;
        }

        user_file.set_mine_tier(tier as u8);
    }

    // add user to mining
    MINING.lock().unwrap().push(sender.clone());

    let ascension = user_file.get_ascension();

    // determine the time
    let time = if user_file.has_super_drill() && ascension != 0 {
        ((BASE_MINE_TIME as f32) / (2.0 * ascension as f32)).min(10.0) as u64
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
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "tier", "The tier you want to mine at").required(false))
        .dm_permission(true)
}