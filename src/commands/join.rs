use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue, UserId};
use crate::{command_response, command_response_loud, GAMES};
use crate::games::{GameCode};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context,
                 command: &CommandInteraction, user: &UserId) {
    let Some(ResolvedOption { value: ResolvedValue::Integer(code, ..), .. }) = options.first() else {
        // error message
        command_response(ctx, &command, "Me confused, Enter the number of the item you wish to equip from your inventory.").await;
        return;
    };

    let code = code.clone() as usize;

    // ensure the code is valid
    if !GAMES.lock().await.code_exists(&(code as GameCode)) {
        // error message
        command_response(ctx, &command, "Invalid game code!").await;
        return;
    }

    let stake = &GAMES.lock().await.get_join_required_info(code as GameCode);

    let mut user_values = UserValues::get(user);

    if let Some(stake) = stake {
        if user_values.get_bananas() < *stake as u64 {
            // error message
            command_response(ctx, &command, "You don't have enough bananas to join this game!").await;
            return;
        }
    }

    // ensure the user is not already in a game
    if GAMES.lock().await.get_player_game(user).is_some() {
        // error message
        command_response(ctx, &command, "You are already in a game!").await;
        return;
    }

    // check if the game is accepting players
    let can_join = GAMES.lock().await.can_join(&(code as GameCode));
    if can_join {
        let join_attempt = GAMES.lock().await.add_player(code as GameCode, user.clone());
        if join_attempt {
            // remove the bananas from the user
            user_values.remove_bananas(stake.unwrap() as u64);

            // success message
            command_response_loud(ctx, &command, "You have joined the game!").await;
            return;
        }
    }

    // error message
    command_response(ctx, &command, "You can't join that game!").await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .description("Join a game")
        .add_option(CreateCommandOption::new(CommandOptionType::Integer,
                                             "code", "the game's code").required(true))
        .dm_permission(false)
}