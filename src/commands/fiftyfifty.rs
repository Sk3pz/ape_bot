use std::sync::atomic::Ordering::SeqCst;
use rand::{Rng, thread_rng};
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
                    CreateInteractionResponseMessage, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::Colour;
use crate::{command_response, hey, SKEPZ_WIN_ALWAYS, SUPERBOOST, SUPERBOOST_MODE};
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, command: &CommandInteraction, user: &UserId) {

    let mut userfile = UserValues::get(user);

    let amt = userfile.get_bananas();

    if amt == 0 {
        command_response(ctx, command, "You have no bananas to bet with!").await;
        return;
    }

    // 50% chance to win
    let win = thread_rng().gen_range(0..2) == 1;

    let embed = if win || (SKEPZ_WIN_ALWAYS.load(SeqCst) && user.get() == 318884828508454912)  {
        let mut winnings = amt;
        if SUPERBOOST_MODE.load(SeqCst) {
            winnings *= SUPERBOOST;
        }
        userfile.add_bananas(winnings);

        CreateEmbed::new()
            .title(format!("50/50 ({} for {}:banana:)", user.to_user(&ctx.http).await.unwrap().global_name.unwrap(), amt))
            .description("Me no like when you win ")
            .color(Colour::GOLD)
            .field("You Win!", format!("You gain {}:banana:!", winnings), false)
            .footer(CreateEmbedFooter::new("Me sad"))
    } else {
        userfile.remove_bananas(amt);
        CreateEmbed::new()
            .title(format!("50/50 ({} for {}:banana:)", user.to_user(&ctx.http).await.unwrap().global_name.unwrap(), amt))
            .description("All your bananas are belong to me")
            .color(Colour::GOLD)
            .field("You Lost!", format!("You lost {}:banana:", amt), false)
            .footer(CreateEmbedFooter::new("Me eat good tonight!"))
    };

    let response = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .add_embed(embed));

    if let Err(e) = command.create_response(&ctx.http, response).await {
        hey!("Error sending message: {:?}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("fiftyfifty")
        .description("All in, 50% chance to double or lose all your bananas")
        .dm_permission(true)
}