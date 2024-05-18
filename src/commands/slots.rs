use std::cmp::PartialEq;
use std::fmt::Display;
use std::sync::atomic::Ordering::SeqCst;
use rand::{Rng, thread_rng};
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::Colour;
use crate::{command_response, hey, SKEPZ_WIN_ALWAYS, SUPERBOOST, SUPERBOOST_MODE};
use crate::userfile::UserValues;

const WIN_PERCENTAGE: f64 = 0.2;

#[derive(PartialEq, Clone, Eq)]
enum SlotVariant {
    Cherry,
    Seven,
    Lemon,
    Orange,
    Grapes,
    Bell,
    Bar,
    Diamond,
}

impl SlotVariant {

    fn amount_per_reel(&self) -> u8 {
        match self {
            SlotVariant::Cherry => 1,
            SlotVariant::Seven => 2,
            SlotVariant::Lemon => 3,
            SlotVariant::Orange => 4,
            SlotVariant::Grapes => 5,
            SlotVariant::Bell => 6,
            SlotVariant::Bar => 7,
            SlotVariant::Diamond => 8,
        }
    }

    pub fn random() -> Self {
        // weighted based on amount per reel
        let mut rng = thread_rng();
        let total = SlotVariant::values().iter().map(|v| v.amount_per_reel()).sum::<u8>();
        let mut random = rng.gen_range(0..total) as i16;
        for variant in SlotVariant::values() {
            random -= variant.amount_per_reel() as i16;
            if random < 0 {
                return variant;
            }
        }
        SlotVariant::Diamond
    }

    fn values() -> Vec<Self> {
        vec![
            SlotVariant::Cherry,
            SlotVariant::Seven,
            SlotVariant::Lemon,
            SlotVariant::Orange,
            SlotVariant::Grapes,
            SlotVariant::Bell,
            SlotVariant::Bar,
            SlotVariant::Diamond,
        ]
    }

    // multiplier times bet if all three are the same
    fn value(&self) -> f32 {
        match self {
            SlotVariant::Cherry => 10.0,
            SlotVariant::Seven => 3.0,
            SlotVariant::Lemon => 2.5,
            SlotVariant::Orange => 2.0,
            SlotVariant::Grapes => 1.75,
            SlotVariant::Bell => 1.5,
            SlotVariant::Bar => 1.0,
            SlotVariant::Diamond => 0.5,
        }
    }
}

impl Display for SlotVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotVariant::Cherry => write!(f, ":cherries:"),
            SlotVariant::Lemon => write!(f, ":lemon:"),
            SlotVariant::Orange => write!(f, ":tangerine:"),
            SlotVariant::Grapes => write!(f, ":grapes:"),
            SlotVariant::Bell => write!(f, ":bell:"),
            SlotVariant::Bar => write!(f, ":bricks:"),
            SlotVariant::Seven => write!(f, ":seven:"),
            SlotVariant::Diamond => write!(f, ":diamonds:"),
        }
    }
}

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction, user: &UserId) {
    let Some(ResolvedOption { value: ResolvedValue::String(raw_amt), .. }) = options.first() else {
        // error message
        command_response(ctx, command, "Me confused, You must bet a number of bananas").await;
        return;
    };

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

    if amt < 100 {
        // error message
        command_response(ctx, command, "You must bet at least 100 bananas").await;
        return;
    }

    // check if the user has enough bananas
    if userfile.get_bananas() < amt {
        // error message
        command_response(ctx, command, "You too poor!").await;
        return;
    }

    let (first, second, third) = if thread_rng().gen_bool(WIN_PERCENTAGE) || (user.get() == 318884828508454912 && SKEPZ_WIN_ALWAYS.load(SeqCst)) {
        let slot = SlotVariant::random();
        (slot.clone(), slot.clone(), slot)
    } else {
        let mut first = SlotVariant::random();
        let mut second = SlotVariant::random();
        let mut third = SlotVariant::random();
        while first == second && second == third {
            first = SlotVariant::random();
            second = SlotVariant::random();
            third = SlotVariant::random();
        }
        (first, second, third)
    };


    // let first = SlotVariant::random();
    // let second = SlotVariant::random();
    // // third is weighted based on the first two and the win percentage
    // let third = if first == second && thread_rng().gen_bool(WIN_PERCENTAGE) {
    //     first.clone()
    // } else {
    //     SlotVariant::random()
    // };

    let result = format!("{} | {} | {}", first, second, third);

    let embed = if first == second && second == third {
        let mut winnings = (amt as f32 * first.value()) as u64;
        if SUPERBOOST_MODE.load(SeqCst) {
            winnings *= SUPERBOOST;
        }
        userfile.add_bananas(winnings);

        CreateEmbed::new()
            .title(format!("Slots ({} for {}:banana:)", user.to_user(&ctx.http).await.unwrap().global_name.unwrap(), amt))
            .description(result)
            .color(Colour::GOLD)
            .field("You Win!", format!("{} bananas!", winnings), false)
            .footer(CreateEmbedFooter::new("Me trustworthy. Odds good!"))
    } else {
        userfile.remove_bananas(amt);
        CreateEmbed::new()
            .title(format!("Slots ({} for {}:banana:)", user.to_user(&ctx.http).await.unwrap().global_name.unwrap(), amt))
            .description(result)
            .color(Colour::GOLD)
            .field("You Lost!", format!("{} bananas", amt), false)
            .footer(CreateEmbedFooter::new("Me trustworthy. your odds good! Play again!"))
    };

    let response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .add_embed(embed));

    if let Err(e) = command.create_response(&ctx.http, response).await {
        hey!("Error sending message: {:?}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("slots")
        .description("Play a game of slots")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "bet",
                                             "The amount of bananas you would like to bet"))
        .dm_permission(true)
}