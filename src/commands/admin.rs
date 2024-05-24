use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, Permissions, ResolvedOption, ResolvedValue};
use crate::{command_response};
use crate::guildfile::GuildSettings;

pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, command: &CommandInteraction, guild: &GuildId) {
    let Some(ResolvedOption { value: ResolvedValue::String(option), .. }) = options.first() else {
        // error message
        command_response(ctx, command, "Me confused, You must specify an option: `add`, `remove`").await;
        return;
    };

    let Some(ResolvedOption { value: ResolvedValue::Channel(channel), .. }) = options.get(1) else {
        // error message
        command_response(ctx, command, "Me confused, You must specify a channel").await;
        return;
    };

    let mut guild_file = GuildSettings::get(guild);

    match option.to_lowercase().as_str() {
        "add" => {
            guild_file.add_channel(channel.id.get());
            command_response(ctx, command, "Channel added to allowed channels").await;
        }
        "remove" => {
            guild_file.remove_channel(channel.id.get());
            command_response(ctx, command, "Channel removed from allowed channels").await;
        }
        _ => {
            // error message
            command_response(ctx, command, "Me confused, You must specify an option: `add`, `remove`").await;
        }
    }

}

pub fn register() -> CreateCommand {
    CreateCommand::new("admin_channel")
        .description("Add and remove channels from Ape Bot's allowed channels (allowed commands)")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "option",
                                             "`add` or `remove`")
            .required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::Channel, "channel",
                                             "The channel to act on")
            .required(true))
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}