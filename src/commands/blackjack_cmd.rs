use serenity::all::{Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, UserId};
use crate::games::blackjack::BlackJack;
use crate::{command_response, GAMES, MINING, nay};
use crate::games::{Games, GameHandler};
use crate::userfile::UserValues;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction, user: &UserId) {
    let Some(ResolvedOption { value: ResolvedValue::String(raw_amt), .. }) = options.first() else {
        // error message
        command_response(ctx, command, "Me confused, You must bet a number of bananas").await;
        return;
    };

    // if the user is mining
    if MINING.lock().unwrap().contains(user) {
        // error message
        command_response(ctx, command, "You are currently mining!").await;
        return;
    }

    let mut userfile = UserValues::get(user);

    // parse out `k` `m` and all
    let raw_amt = raw_amt.to_lowercase();
    let amt = if raw_amt == "all" {
        userfile.get_bananas()
    } else if raw_amt == "half" {
        userfile.get_bananas() / 2
    } else if raw_amt.ends_with("k") {
        raw_amt[..raw_amt.len() - 1].parse::<u64>().unwrap_or(5) * 1000
    } else if raw_amt.ends_with("m") {
        raw_amt[..raw_amt.len() - 1].parse::<u64>().unwrap_or(5) * 1000000
    } else {
        let Ok(parse) = raw_amt.parse::<u64>() else {
            // error message
            command_response(ctx, command, "Me confused, You must bet a number of bananas").await;
            return;
        };
        parse
    };

    if amt < 5 {
        // error message
        command_response(ctx, command, "You must bet at least 5 bananas").await;
        return;
    }

    // check if the user has enough bananas
    if userfile.get_bananas() < amt {
        // error message
        command_response(ctx, command, "You too poor!").await;
        return;
    }

    // if the user is already in a game
    if GAMES.lock().unwrap().get_player_game(user).is_some() {
        // error message
        command_response(ctx, command, "You are already in a game!").await;
        return;
    }

    // remove the nanners from the user
    userfile.remove_bananas(amt);

    // create the blackjack game
    let mut game = BlackJack::new(amt);
    game.deal();

    if game.player_blackjack {
        let embed = CreateEmbed::new()
            .title(format!("Blackjack - {} Bananas for {}", amt, user.to_user(&ctx.http).await.unwrap().global_name.unwrap()))
            .thumbnail("attachment://monkey.png")
            .description("George has dealt the cards - **YOU GOT BLACKJACK!**")
            .color(Colour::GOLD)
            .field(format!("Your Hand ({})", game.player.playing_hand().score()), format!("{}", game.player.playing_hand()), false)
            .field("George's hand".to_string(), format!("{} ({})", game.dealer_card(),
                                                        game.dealer_card().display_no_suite()), false)
            .field("You won!", "Go you!", false)
            .footer(CreateEmbedFooter::new(format!("George Advice: {}", game.give_help())));

        let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .embed(embed)
            .add_file(CreateAttachment::path("./images/monkey.png").await.unwrap()));

        if let Err(e) = command.create_response(&ctx.http, msg).await {
            // error message
            command_response(ctx, command, "Failed to respond to command: {}").await;
            nay!("Failed to send bj message: {}", e);
        }
        // add the bet back and 2.5 times
        userfile.add_bananas(amt + (amt as f64 * 2.5) as u64);

        return;
    }

    if game.offered_insurance {
        let embed = CreateEmbed::new()
            .title(format!("Blackjack - {} Bananas for {}", amt, user.to_user(&ctx.http).await.unwrap().global_name.unwrap()))
            .thumbnail("attachment://monkey.png")
            .description("George has dealt the cards")
            .color(Colour::GOLD)
            .field(format!("Your Hand ({})", game.player.playing_hand().score()), format!("{}", game.player.playing_hand()), false)
            .field("George's hand".to_string(), format!("{} ({})", game.dealer_card(),
                                                      game.dealer_card().display_no_suite()), false)
            .field("Would you like insurance?", "`yes` or `no`", false)
            .footer(CreateEmbedFooter::new(format!("George Advice: {}", game.give_help())));

        let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .embed(embed)
            .add_file(CreateAttachment::path("./images/monkey.png").await.unwrap()));

        if let Err(e) = command.create_response(&ctx.http, msg).await {
            // error message
            command_response(ctx, command, "Failed to respond to command: {}").await;
            nay!("Failed to send bj message: {}", e);
        }

        GAMES.lock().unwrap().insert(GameHandler::new(user.clone(), Games::BlackJack(game)));
        return;
    }

    // show the first embed
    let embed = CreateEmbed::new()
        .title(format!("Blackjack - {} Bananas for {}", amt, user.to_user(&ctx.http).await.unwrap().global_name.unwrap()))
        .thumbnail("attachment://monkey.png")
        .description("George has dealt the cards")
        .color(Colour::GOLD)
        .field(format!("Your Hand ({})", game.player.playing_hand().score()), format!("{}", game.player.playing_hand()), false)
        .field("George's hand".to_string(), format!("{} ({})", game.dealer_card(),
                                                    game.dealer_card().display_no_suite()), false)
        .field("Options", format!("{}", game.give_options()), false)
        .footer(CreateEmbedFooter::new(format!("George Advice: {}", game.give_help())));

    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                                                     .embed(embed)
        .add_file(CreateAttachment::path("./images/monkey.png").await.unwrap()));

    if let Err(e) = command.create_response(&ctx.http, msg).await {
        // error message
        command_response(ctx, command, "Failed to respond to command: {}").await;
        nay!("Failed to send bj message: {}", e);
    }

    GAMES.lock().unwrap().insert(GameHandler::new(user.clone(), Games::BlackJack(game)));
}

pub fn register() -> CreateCommand {
    CreateCommand::new("blackjack")
        .description("Bet against the monkey in a hand of blackjack")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "bet",
                                             "The amount of bananas you would like to bet")
            .required(true))
        .dm_permission(true)
}