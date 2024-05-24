use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use better_term::{Color, Style};
use lazy_static::lazy_static;
use rand::{Rng, thread_rng};
use serenity::all::{ActivityData, ChannelId, Colour, Command, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GatewayIntents, Interaction, Member, Mentionable, Message, OnlineStatus, PartialGuild, Ready, ResumedEvent, Timestamp, UserId, VoiceState};
use serenity::{async_trait, Client};
use serenity::client::EventHandler;
use crate::commands::{admin, banana, blackjack_cmd, buy, collect_minions, discard, fiftyfifty, help, inventory_cmd, mine, shop, slots};
use crate::games::{GamesManager};
use crate::mine_data::Mine;

pub mod logging;
pub mod userfile;
pub mod guildfile;
mod commands;
pub mod games;
mod inventory;
mod mine_data;

lazy_static!(
    static ref CRATE_ACTIVE: Mutex<AtomicBool> = Mutex::new(AtomicBool::new(false));
    static ref CRATE_CODE: Mutex<String> = Mutex::new(String::new());

    static ref USERS_IN_VOICE: Mutex<Vec<(UserId, Timestamp)>> = Mutex::new(Vec::new());

    static ref GAMES: Mutex<GamesManager> = Mutex::new(GamesManager::new());

    static ref MINING: Mutex<Vec<UserId>> = Mutex::new(Vec::new());

    static ref SUPERBOOST_MODE: AtomicBool = AtomicBool::new(false);
    static ref SKEPZ_WIN_ALWAYS: AtomicBool = AtomicBool::new(false);
);

const CODE_LENGTH: u8 = 6;
const MSG_BANANA_GAIN_MIN: u64 = 5;
const MSG_BANANA_GAIN_MAX: u64 = 25;
const VOICE_MINUTE_BANANA_WORTH: u64 = 150;

pub const SLUDGE_BANANA_WORTH: u64 = 250; // produce 1-10 sludge by default per mining

const SUPERBOOST: u64 = 2;

// function that runs every minute to give bananas to users in voice channels
pub async fn voice_minute_banana() {
    loop {
        // sleep for 1 minute
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

        let users_in_voice = USERS_IN_VOICE.lock().unwrap();
        for (id, time) in &*users_in_voice {
            let full_dur = Timestamp::now().signed_duration_since(time.fixed_offset()).num_seconds();
            let duration = full_dur.min(60) as f32 / 60.0;
            let mut bananas = (duration * VOICE_MINUTE_BANANA_WORTH as f32) as u64;
            if SUPERBOOST_MODE.load(Ordering::SeqCst) {
                bananas *= SUPERBOOST;
            }
            let mut userfile = userfile::UserValues::get(id);
            userfile.add_bananas(bananas);
        }
    }
}

pub async fn is_admin(guild: &PartialGuild, member: &Member) -> bool {
    for r in &member.roles {
        // get the role:
        if let Some(role) = guild.roles.get(r) {
            if role.permissions.administrator() {
                return true;
            }
        }
    }

    return false;
}

pub fn is_supreme_overlord(id: UserId) -> bool {
    id == 318884828508454912
}

pub fn gen_crate_code() -> String {
    let mut code = String::new();

    for x in 0..CODE_LENGTH {
        code.push((thread_rng().gen_range(33_u8..=126_u8) as char).to_ascii_uppercase());
        if x != CODE_LENGTH - 1 {
            code.push(' ');
        }
    }

    code = code.replace("`", "'");
    code = code.replace("\\", "/");

    code
}

pub async fn spawn_crate(ctx: &Context, channel: &ChannelId) {
    let code = gen_crate_code();
    CRATE_CODE.lock().unwrap().clear();
    CRATE_CODE.lock().unwrap().push_str(&code);
    CRATE_ACTIVE.lock().unwrap().store(true, Ordering::SeqCst);

    // send the embed
    let embed = CreateEmbed::new()
        .title("Banana Crate")
        .color(Colour::DARK_GOLD)
        .description("A crate of bananas has spawned!")
        .thumbnail("attachment://crate.jpg")
        .field("Type the code first to get the crate!", format!("Code: `{}`", code.clone()), true)
        .timestamp(Timestamp::now());
    let message = CreateMessage::new()
        .add_file(CreateAttachment::path("./images/crate.jpg").await.unwrap())
        .embed(embed);
    if let Err(e) = channel.send_message(&ctx.http, message).await {
        nay!("Failed to send crate message: {}", e);
    }
}

pub async fn command_response<S: Into<String>>(ctx: &Context, command: &CommandInteraction, msg: S) {
    let data = CreateInteractionResponseMessage::new().content(msg.into()).ephemeral(true);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(err) = command.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub async fn command_response_loud<S: Into<String>>(ctx: &Context, command: &CommandInteraction, msg: S) {
    let data = CreateInteractionResponseMessage::new().content(msg.into());
    let builder = CreateInteractionResponse::Message(data);
    if let Err(err) = command.create_response(&ctx.http, builder).await {
        nay!("Failed to respond to command: {}", err)
    }
}

pub async fn register_command(ctx: &Context, cmd: CreateCommand) {
    if let Err(e) = Command::create_global_command(&ctx.http, cmd).await {
        nay!("Failed to register a command: {}", e);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from bots
        if msg.author.bot {
            return;
        }

        let channel = msg.channel_id;

        let user = &msg.author;
        let mut userfile = userfile::UserValues::get(&user.id);

        if is_supreme_overlord(user.id) {
            if msg.content == "superboostmode" {
                SUPERBOOST_MODE.store(!SUPERBOOST_MODE.load(Ordering::SeqCst), Ordering::SeqCst);
                let embed = if SUPERBOOST_MODE.load(Ordering::SeqCst) {
                    CreateEmbed::new()
                        .title("SUPER BOOST MODE")
                        .description(format!("x{} bananas on **ALL** gains!", SUPERBOOST))
                        .color(Colour::GOLD)
                        .thumbnail("attachment://george.png")
                        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
                } else {
                    CreateEmbed::new()
                        .title("No more boost mode")
                        .description("Back to regular gains".to_string())
                        .color(Colour::GOLD)
                        .thumbnail("attachment://george.png")
                        .footer(CreateEmbedFooter::new("Brought to you by A.P.E. Inc©"))
                };

                let msg = CreateMessage::new()
                    .embed(embed)
                    .add_file(CreateAttachment::path("./images/george.png").await.unwrap());

                if let Err(e) = channel.send_message(&ctx.http, msg).await {
                    nay!("Failed to send message: {}", e);
                }
                return;
            }
            if msg.content == "ez" {
                SKEPZ_WIN_ALWAYS.store(!SKEPZ_WIN_ALWAYS.load(Ordering::SeqCst), Ordering::SeqCst);
                return;
            }
            if msg.content.starts_with("gsd") {
                let recipient = msg.mentions.get(0);
                if recipient.is_none() {
                    return;
                }

                let recipient = recipient.unwrap();
                let mut recipient_file = userfile::UserValues::get(&recipient.id);
                recipient_file.add_super_drill();
            }
        }

        // handle if the user is in a game
        let (embed, thumbnail_path) = {
            // ensure the channel is a spam channel
            let mut guild_file = guildfile::GuildSettings::get(&msg.guild_id.unwrap());
            if !guild_file.is_allowed_channel(channel.get()) {
                (None, None)
            } else {
                if !msg.content.is_empty() {
                    let mut lock = GAMES.lock().expect("Failed to get games lock");
                    if let Some(code) = lock.get_player_game(&user.id) {
                        let game = lock.get_game(code).unwrap();
                        match game.game {
                            games::Games::BlackJack(ref mut bj) => {
                                let (embed, end) = bj.handle_message(&msg);

                                if end {
                                    lock.end_game(code);
                                }

                                (Some(embed), Some("./images/monkey.png".to_string()))
                            }
                            games::Games::SludgeMonsterBattle(ref mut battle) => {
                                let (mut embed, end) = battle.handle_message(&msg);

                                let thumbnail_path = battle.thumbnail.clone();

                                embed = embed.thumbnail(format!("attachment://{}", thumbnail_path));

                                if end {
                                    lock.end_game(code);
                                }

                                (Some(embed), Some(format!("./images/sludge_monsters/{}", thumbnail_path)))
                            }
                            games::Games::MineBattle(ref mut battle) => {
                                let (mut embed, end) = battle.handle_message(&msg);

                                let thumbnail = battle.enemy.thumbnail.clone();

                                embed = embed.thumbnail(format!("attachment://{}", thumbnail));

                                if end {
                                    lock.end_game(code);
                                }

                                (Some(embed), Some(format!("./images/sludge_monsters/{}", thumbnail)))
                            }
                            games::Games::TexasHoldem(ref mut th) => {
                                let (embed, end) = th.handle_message(&msg);

                                if end {
                                    lock.end_game(code);
                                }

                                (Some(embed), None)
                            }
                        }
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                }
            }
        };

        if let Some(embed) = embed {
            let mut builder = CreateMessage::new()
                .content(format!("{}", user.mention()))
                .embed(embed)
                .reference_message(&msg);

            if let Some(thumbnail_path) = thumbnail_path {
                builder = builder.add_file(CreateAttachment::path(format!("{}", thumbnail_path)).await.unwrap());
            }

            if let Err(e) = channel.send_message(&ctx.http, builder).await {
                nay!("Failed to send message: {}", e);
            }
            return;
        }

        // check if there is an active crate and if so, did the user type the correct code?
        if CRATE_ACTIVE.lock().unwrap().load(Ordering::SeqCst) {
            let needed_code = CRATE_CODE.lock().unwrap().replace(" ", "");
            if msg.content.to_ascii_uppercase() == needed_code {
                // second check for messages that are way too close
                if !CRATE_ACTIVE.lock().unwrap().load(Ordering::SeqCst) {
                    return;
                }
                CRATE_ACTIVE.lock().unwrap().store(false, Ordering::SeqCst);
                let bananas = thread_rng().gen_range(10..250);
                if let Err(e) = msg.reply(&ctx.http, format!("You've opened the crate and found {} bananas!", bananas)).await {
                    nay!("Failed to send crate reward message: {}", e);
                }
                userfile.add_bananas(bananas);
            }
            return;
        }

        let mut gained_bananas = thread_rng().gen_range(MSG_BANANA_GAIN_MIN..=MSG_BANANA_GAIN_MAX);

        // if the message contains an embed, multiply the gained bananas by 2
        if msg.embeds.len() > 0 {
            gained_bananas *= 2;
        }

        if SUPERBOOST_MODE.load(Ordering::SeqCst) {
            gained_bananas *= SUPERBOOST;
        }

        let msg_content = msg.content.to_ascii_lowercase();

        if msg_content == "hi george" || msg_content == "hello george" || msg_content == "hey george" || msg_content == "hello, george"
        || msg_content == "hi george!" || msg_content == "hello george!" || msg_content == "hey george!" || msg_content == "hello, george!" {
            let image_path = "./images/george.png".to_string();
            let reply = CreateMessage::new()
                .content("Hello there!")
                .add_file(CreateAttachment::path(image_path).await.unwrap())
                .reference_message(&msg);
            if let Err(e) = channel.send_message(&ctx.http, reply).await {
                nay!("Failed to send message: {}", e);
            }
            return;
        }

        if msg_content.contains("bad bot") || msg_content.contains("bad monkey") || msg_content.contains("bad ape") {
            let image_path = "./images/mad_gorilla.jpg".to_string();
            let reply = CreateMessage::new()
                .content("Look who's talking")
                .add_file(CreateAttachment::path(image_path).await.unwrap())
                .reference_message(&msg);
            if let Err(e) = channel.send_message(ctx.http, reply).await {
                nay!("Failed to send message: {}", e);
            }
            return;
        }

        if msg_content.contains("odds") {
            let image_path = "./images/george.png".to_string();
            let reply = CreateMessage::new()
                .content("You talk about odds? My games are fair, I swear!")
                .add_file(CreateAttachment::path(image_path).await.unwrap())
                .reference_message(&msg);
            if let Err(e) = channel.send_message(&ctx.http, reply).await {
                nay!("Failed to send message: {}", e);
            }
        }

        userfile.add_bananas(gained_bananas);

        // random monkey image check
        if thread_rng().gen_range(0..1000) == 0 {
            let image_path = format!("./images/random/monke{}.jpg", thread_rng().gen_range(1..=16));
            let reply = CreateMessage::new()
                .content("It's random monkey image time!! :tada: :tada: :tada:")
                .add_file(CreateAttachment::path(image_path).await.unwrap());
            if let Err(e) = channel.send_message(&ctx.http, reply).await {
                nay!("Failed to send message: {}", e);
            }
            return;
        }

        // crate check
        if thread_rng().gen_range(0..100) == 0 {
            spawn_crate(&ctx, &channel).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // register commands here
        register_command(&ctx, help::register()).await;
        register_command(&ctx, banana::info::register()).await;
        register_command(&ctx, banana::levelup::register()).await;
        register_command(&ctx, banana::prestige::register()).await;
        register_command(&ctx, banana::leaderboard::register()).await;
        register_command(&ctx, blackjack_cmd::register()).await;
        register_command(&ctx, banana::pay::register()).await;
        register_command(&ctx, banana::ascend::register()).await;
        register_command(&ctx, slots::register()).await;
        register_command(&ctx, fiftyfifty::register()).await;
        register_command(&ctx, mine::register()).await;
        register_command(&ctx, inventory_cmd::register()).await;
        register_command(&ctx, shop::register()).await;
        register_command(&ctx, buy::register()).await;
        register_command(&ctx, discard::register()).await;
        register_command(&ctx, collect_minions::register()).await;

        register_command(&ctx, admin::register()).await;

        yay!("{} is connected!", ready.user.name);
        ctx.set_presence(Some(ActivityData::playing("with banana")), OnlineStatus::Online);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        hey!("Resumed");
    }

    async fn voice_state_update(&self, _ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        // if old is none and new.channel_id is some, the user joined the voice channel
        if old.is_none() && new.channel_id.is_some() {
            if new.user_id.to_user(&_ctx.http).await.unwrap().bot {
                return;
            }
            // USER JOINED
            // lock the users in voice vector
            let mut users_in_voice = USERS_IN_VOICE.lock().unwrap();
            // push the user id if they are not already in
            if !users_in_voice.iter().any(|(id,_)| *id == new.user_id) {
                users_in_voice.push((new.user_id, Timestamp::now()));
            }
        }
        // if old is some and new.channel_id is none, the user left the voice channel
        else if old.is_some() && new.channel_id.is_none() {
            if new.user_id.to_user(&_ctx.http).await.unwrap().bot {
                return;
            }
            // USER LEFT
            // lock the users in voice vector
            let mut users_in_voice = USERS_IN_VOICE.lock().unwrap();
            // remove the user id
            if let Some(index) = users_in_voice.iter().position(|(id,_)| *id == new.user_id) {
                let _ = users_in_voice.remove(index);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let command_name = command.data.name.as_str();
                let sender = &command.user;
                let guild = command.guild_id;
                if guild.is_none() {
                    command_response(&ctx, &command, "This command can only be used in a server").await;
                    return;
                }
                let guild_id = guild.unwrap();
                let command_options = &command.data.options();
                let channel = command.channel_id;

                let mut guild_settings = guildfile::GuildSettings::get(&guild_id);

                // non-channel specific commands
                match command_name {
                    "help" => {
                        help::run(&ctx, &command).await;
                        return;
                    }
                    "info" => {
                        banana::info::run(&ctx, &command, &sender.id).await;
                        return;
                    }
                    "levelup" => {
                        banana::levelup::run(command_options, &ctx, &command, &sender.id).await;
                        return;
                    }
                    "prestige" => {
                        banana::prestige::run(&ctx, &command, &sender.id).await;
                        return;
                    }
                    "leaderboard" => {
                        banana::leaderboard::run(&ctx, &command).await;
                        return;
                    }
                    "pay" => {
                        banana::pay::run(command_options, &ctx, &command, &sender.id).await;
                        return;
                    }
                    "ascend" => {
                        banana::ascend::run(&ctx, &command, &sender.id).await;
                        return;
                    }
                    "inventory" => {
                        inventory_cmd::run(&ctx, &command).await;
                        return;
                    }
                    "shop" => {
                        shop::run(&ctx, &command).await;
                        return;
                    }
                    "buy" => {
                        buy::run(command_options, &ctx, &command).await;
                        return;
                    }
                    "discard" => {
                        discard::run(command_options, &ctx, &command).await;
                        return;
                    }
                    "collect_minions" => {
                        collect_minions::run(&ctx, &command).await;
                        return;
                    }
                    "admin_channel" => {
                        admin::run(command_options, &ctx, &command, &guild_id).await;
                        return;
                    }
                    _ => {}
                }

                // spam commands that need checked
                if guild_settings.is_allowed_channel(channel.get()) {
                    match command_name {
                        "blackjack" => {
                            blackjack_cmd::run(command_options, &ctx, &command, &sender.id).await;
                        }
                        "slots" => {
                            slots::run(command_options, &ctx, &command, &sender.id).await;
                        }
                        "fiftyfifty" => {
                            fiftyfifty::run(&ctx, &command, &sender.id).await;
                        }
                        "mine" => {
                            mine::run(command_options, &ctx, &channel, command.clone(), &sender.id).await;
                        }
                        _ => {
                            command_response(&ctx, &command, "That do be a monkey brain moment").await;
                        }
                    }
                } else {
                    let allowed_channels = {
                        let mut channels = Vec::new();
                        for cid in guild_settings.get_channels() {
                            channels.push(cid.to_channel(&ctx).await.unwrap().to_string());
                        }
                        channels
                    };
                    command_response(&ctx, &command, format!("That command is only allowed in the following channel(s): {}", allowed_channels.join(", "))).await;
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let monkey =
        r#"|          ."`".          |
|      .-./ _=_ \.-.      |
|     {  (,(oYo),) }}     |
|     {{ |   "   |} }     |
|     { { \(---)/  }}     |
|     {{  }'-=-'{ } }     |
|     { { }._:_.{  }}     |
|     {{  } -:- { } }     |
|     {_{ }`===`{  _}     |
|    ((((\)     (/))))    |
|  Hi! My name is George  |"#;
    println!("{}+-------{}APE BOT V-2{}-------+\n\
    {}{}\n\
    {}+-------------------------+", Color::White, Style::default().bold().fg(Color::Green),
             Style::reset().fg(Color::White), Color::BrightBlack, monkey, Color::White);

    dotenv::dotenv().expect("Failed to load .env file");

    let Ok(token) = env::var("DISCORD_TOKEN") else {
        nay!("DISCORD_TOKEN not found in environment");
        return;
    };

    /*
        Ensure the following directories exist:
        ./guilds
        ./users
    */
    let paths = vec!["./guilds", "./users"];
    for path in paths {
        if !std::path::Path::new(path).exists() {
            std::fs::create_dir(path).expect("Failed to create directory");
        }
    }

    // ensure all mine tier files are valid before loading the bot
    let _mine = Mine::get();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS;

    // spawn an async thread for the voice minute banana function
    tokio::spawn(voice_minute_banana());

    let Ok(mut client) = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        else {
            nay!("Error creating client");
            return;
        };

    if let Err(err) = client.start().await {
        nay!("Client error: {}", err);
    }
}
