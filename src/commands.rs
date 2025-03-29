use pumpkin::{
    command::{
        args::{bool::BoolArgConsumer, simple::SimpleArgConsumer, Arg, ConsumedArgs},
        dispatcher::CommandError,
        tree::{builder::argument, CommandTree},
        CommandExecutor, CommandSender,
    },
    server::Server,
};
use pumpkin_util::text::{color::NamedColor, TextComponent};

use crate::{save_config, CommandInfo, CONFIG};

const NAMES: [&str; 3] = ["limitcommand", "limit", "lc"];
const DESCRIPTION: &str = "Limits the use of a command to certain players.";
const ARG_COMMAND: &str = "command";
const ARG_BLACKLIST: &str = "blacklist";
const ARG_ALLOWED: &str = "allowed";

struct BlacklistAll;

#[async_trait::async_trait]
impl CommandExecutor for BlacklistAll {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Msg(command)) = args.get(ARG_COMMAND) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_COMMAND.into())));
        };

        let mut config = CONFIG.lock().await.clone();

        for mut c in config.commands.clone() {
            if c.name == *command {
                c.blacklist = false;
                c.allowed = Vec::new();

                sender
                    .send_message(
                        TextComponent::text("Successfully blacklisted command.")
                            .color_named(NamedColor::Green),
                    )
                    .await;

                let _ = save_config().await;

                return Ok(());
            }
        }

        config.commands.push(CommandInfo {
            name: command.clone(),
            blacklist: false,
            allowed: Vec::new(),
        });

        sender
            .send_message(
                TextComponent::text("Successfully blacklisted command.")
                    .color_named(NamedColor::Green),
            )
            .await;

        let _ = save_config().await;

        Ok(())
    }
}

struct LimitCommand;

#[async_trait::async_trait]
impl CommandExecutor for LimitCommand {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Msg(command)) = args.get(ARG_COMMAND) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_COMMAND.into())));
        };

        let Some(Arg::Bool(blacklist)) = args.get(ARG_BLACKLIST) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_BLACKLIST.into())));
        };

        let Some(Arg::Msg(allowed)) = args.get(ARG_ALLOWED) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_ALLOWED.into())));
        };

        let mut config = CONFIG.lock().await.clone();

        for mut c in config.commands.clone() {
            if c.name == *command {
                c.blacklist = *blacklist;
                c.allowed = allowed.split(',').map(|s| s.to_string()).collect();

                sender
                    .send_message(
                        TextComponent::text("Successfully limited command.")
                            .color_named(NamedColor::Green),
                    )
                    .await;

                let _ = save_config().await;

                return Ok(());
            }
        }

        config.commands.push(CommandInfo {
            name: command.clone(),
            blacklist: *blacklist,
            allowed: allowed.split(',').map(|s| s.to_string()).collect(),
        });

        sender
            .send_message(
                TextComponent::text("Successfully limited command.").color_named(NamedColor::Green),
            )
            .await;

        let _ = save_config().await;

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        argument(ARG_COMMAND, SimpleArgConsumer)
            .execute(BlacklistAll)
            .then(
                argument(ARG_BLACKLIST, BoolArgConsumer)
                    .then(argument(ARG_ALLOWED, SimpleArgConsumer).execute(LimitCommand)),
            ),
    )
}
