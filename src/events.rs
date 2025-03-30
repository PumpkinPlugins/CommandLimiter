use pumpkin::plugin::{
    Cancellable, EventHandler,
    player::{PlayerEvent, player_command_send::PlayerCommandSendEvent},
};
use pumpkin_util::text::{TextComponent, color::NamedColor};

use crate::CONFIG;

pub struct CommandSendHandler;

#[async_trait::async_trait]
impl EventHandler<PlayerCommandSendEvent> for CommandSendHandler {
    async fn handle_blocking(&self, event: &mut PlayerCommandSendEvent) {
        let config = CONFIG.lock().await.clone();
        let command = event.command.clone();
        let player = event.get_player().gameprofile.name.clone();

        for cmd in config.commands.iter() {
            if cmd.name == command {
                let should_block = if cmd.blacklist {
                    cmd.allowed.contains(&player)
                } else {
                    !cmd.allowed.contains(&player)
                };

                if should_block {
                    event.set_cancelled(true);
                    event
                        .get_player()
                        .send_system_message(
                            &TextComponent::text(config.block_message).color_named(NamedColor::Red),
                        )
                        .await;
                }
                return;
            }
        }
    }
}
