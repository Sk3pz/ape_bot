use serenity::all::{Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, UserId};
use crate::{command_response, nay, GAMES};
use crate::games::{GameHandler, Games};
use crate::games::pvp::{PvPArena, PvPModFlag};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context,
                 command: &CommandInteraction, user: &UserId) {

    let mut stake = 0;
    let mut public = true;

    let mut max_players = 2;
    let mut base_health = 100;
    let mut max_health = 100;
    let mut no_items = false;

    for optn in options {
        match optn {
            ResolvedOption { name, value: ResolvedValue::Integer(val), .. } => {
                match *name {
                    "stake" => stake = *val as u64,
                    "max_players" => max_players = *val as u64,
                    "base_health" => base_health = *val as u32,
                    "max_health" => max_health = *val as u32,
                    _ => {}
                }
            },
            ResolvedOption { name, value: ResolvedValue::Boolean(val), .. } => {
                match *name {
                    "public" => public = *val,
                    "no_items" => no_items = *val,
                    _ => {}
                }
            },
            _ => {}
        }
    }

    // if stake < 5 {
    //     command_response(ctx, command, "You must bet at least 5 bananas").await;
    //     return;
    // }

    // check that the user has enough bananas
    let mut userfile = UserValues::get(user);

    if userfile.get_bananas() < stake {
        command_response(ctx, command, "You too poor!").await;
        return;
    }

    // setup game flags
    let mut flags = vec![
        PvPModFlag::MaxPlayers(max_players),
        PvPModFlag::CustomStartHealth(base_health),
        PvPModFlag::CustomMaxHealth(max_health),
        ];
    if no_items {
        flags.push(PvPModFlag::NoItem);
    }

    // ensure the user is not already in a game
    if GAMES.lock().unwrap().get_player_game(user).is_some() {
        command_response(ctx, command, "You are already in a game!").await;
        return;
    }

    // remove the stake from the user's balance
    userfile.remove_bananas(stake);

    // create the game
    let game = PvPArena::new(user.clone(), stake, flags);

    // add user to the game handler
    let code = GAMES.lock().unwrap().insert(GameHandler::new(user.clone(), Games::PvP(game)));

    // display the embed
    let embed = CreateEmbed::new()
        .title(format!("{}'s PvP Arena", user.to_user(&ctx.http).await.unwrap().global_name.unwrap()))
        .description(format!("**Game code: `{}`**\nOther users can join with *`/join`*", code))
        .thumbnail("attachment://battle_monkey.jpeg")
        .color(Colour::RED)
        .fields(vec![
            ("Stake", format!("{}:banana:", stake), true),
            ("Max Players", format!("{}", max_players), true),
            ("Public", format!("{}", public), true),
            ("Base Health", format!("{}", base_health), true),
            ("Max Health", format!("{}", max_health), true),
            ("No Items", format!("{}", no_items), true),
        ])
        .footer(CreateEmbedFooter::new("Type `start` to start, `end` to cancel the game, `list` to see players, `leave` to leave, or `kick` to kick a player"));

    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(!public)
        .add_file(CreateAttachment::path("./images/battle_monkey.jpeg").await.unwrap()));

    if let Err(e) = command.create_response(&ctx.http, msg).await {
        // error message
        command_response(ctx, command, "Failed to respond to command: {}").await;
        nay!("Failed to send pvp message: {}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pvp")
        .description("Start a PvP arena")
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "stake", "The loosing amount (min 5)").required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "max_players", "Max players that can join (default 2)").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Boolean,
                                             "public", "Set if the code can be seen by everyone (default true)").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Boolean,
                                             "no_items", "Disable items if true (default: false)").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "base_health", "Starting health of all players (default 100)").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "max_health", "The maximum health you can heal to (default 100)").required(false))
        .dm_permission(true)
}