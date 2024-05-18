use std::fs;
use serenity::all::{Colour, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, UserId};
use crate::{hey, nay};
use crate::userfile::UserValues;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    // get the files in the users directory
    let paths = fs::read_dir("./users").unwrap();

    let mut top_users: Vec<UserValues> = Vec::new();
    for path in paths {
        if let Ok(path) = path {
            let user_id = path.file_name().into_string().unwrap().replace(".json", "");
            // check if user_id is numeric
            if user_id.parse::<u64>().is_ok() {
                let user_id = UserId::from(user_id.parse::<u64>().unwrap());
                let user = UserValues::get(&user_id);
                if top_users.len() < 3 {
                    top_users.push(user.clone());
                }

                if top_users.len() == 3 {
                    if user.file.ascension > top_users[2].file.ascension {
                        top_users[2] = user.clone();
                    } else {
                        if user.file.prestige > top_users[2].file.prestige {
                            top_users[2] = user.clone();
                        } else if user.file.prestige == top_users[2].file.prestige {
                            // whoever has the higher level
                            if user.file.level > top_users[2].file.level {
                                top_users[2] = user.clone();
                            }
                        }
                    }
                }

                if top_users.len() >= 2 {
                    if user.file.ascension > top_users[1].file.ascension {
                        top_users[2] = top_users[1].clone();
                        top_users[1] = user.clone();
                    } else {
                        if user.file.prestige > top_users[1].file.prestige {
                            top_users[2] = top_users[1].clone();
                            top_users[1] = user.clone();
                        } else if user.file.prestige == top_users[1].file.prestige {
                            // whoever has the higher level
                            if user.file.level > top_users[1].file.level {
                                top_users[2] = top_users[1].clone();
                                top_users[1] = user.clone();
                            }
                        }
                    }
                }

                if top_users.len() >= 1 {
                    if user.file.ascension > top_users[0].file.ascension {
                        top_users[1] = top_users[0].clone();
                        top_users[0] = user.clone();
                    } else {
                        if user.file.prestige > top_users[0].file.prestige {
                            top_users[1] = top_users[0].clone();
                            top_users[0] = user.clone();
                        } else if user.file.prestige == top_users[0].file.prestige {
                            // whoever has the higher level
                            if user.file.level > top_users[0].file.level {
                                top_users[1] = top_users[0].clone();
                                top_users[0] = user.clone();
                            }
                        }
                    }
                }
            } else {
                hey!("Failed to parse for leaderboard!");
            }
        } else {
            hey!("Failed to read user file for leaderboard!");
        }
    }

    let n1 = if top_users.len() > 0 {
        (UserId::from(top_users[0].id).to_user(&ctx.http).await.unwrap().global_name.unwrap_or("UNKNOWN".to_string()),
         top_users[0].file.ascension,
         top_users[0].file.prestige.to_string(), top_users[0].file.level.to_string())
    } else {
        ("UNKNOWN".to_string(), 0, "?".to_string(), "?".to_string())
    };
    let n2 = if top_users.len() > 1 {
        (UserId::from(top_users[1].id).to_user(&ctx.http).await.unwrap().global_name.unwrap_or("UNKNOWN".to_string()),
         top_users[1].file.ascension,
         top_users[1].file.prestige.to_string(), top_users[1].file.level.to_string())
    } else {
        ("UNKNOWN".to_string(), 0, "?".to_string(), "?".to_string())
    };
    let n3 = if top_users.len() > 2 {
        (UserId::from(top_users[2].id).to_user(&ctx.http).await.unwrap().global_name.unwrap_or("UNKNOWN".to_string()),
         top_users[2].file.ascension,
         top_users[2].file.prestige.to_string(), top_users[2].file.level.to_string())
    } else {
        ("UNKNOWN".to_string(), 0, "?".to_string(), "?".to_string())
    };

    // craft the embed
    let embed = CreateEmbed::default()
        .title("Top Users")
        .color(Colour::GOLD)
        .thumbnail("attachment://ape.png")
        .field("1st (Nanner King)", format!("**{}:** {} Prestige: `{}` Level: `{}`", n1.0,
                                            if n1.1 != 0 { format!("Ascention: `{}`", n1.1.to_string()) } else { "".to_string() },
                                            n1.2, n1.3), false)
        .field("2nd", format!("**{}:** {} Prestige: `{}` Level: `{}`", n2.0,
                              if n2.1 != 0 { format!("Ascention: `{}`", n2.1.to_string()) } else { "".to_string() },
                              n2.2, n2.3), false)
        .field("3rd", format!("**{}:** {} Prestige: `{}` Level: `{}`", n3.0,
                              if n3.1 != 0 { format!("Ascention: `{}`", n3.1.to_string()) } else { "".to_string() },
                              n3.2, n3.3), false);

    // send the embed
    let msg = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .embed(embed)
        .add_file(CreateAttachment::path("./images/ape.png").await.unwrap())
    );

    if let Err(e) = cmd.create_response(&ctx.http, msg).await {
        nay!("Failed to respond to command: {}", e);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leaderboard")
        .description("View the top users")
        .dm_permission(true)
}