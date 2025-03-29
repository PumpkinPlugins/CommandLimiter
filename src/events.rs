use pumpkin::plugin::{
    player::player_command_send::PlayerCommandSendEvent, Cancellable, EventHandler,
};

use crate::CONFIG;

pub struct CommandSendHandler;

#[async_trait::async_trait]
impl EventHandler<PlayerCommandSendEvent> for CommandSendHandler {
    async fn handle_blocking(&self, event: &mut PlayerCommandSendEvent) {
        let config = CONFIG.lock().await.clone();
        let command = event.command.clone();
        let player = event.player.gameprofile.name.clone();

        for cmd in config.commands.iter() {
            if cmd.name == command {
                if cmd.blacklist {
                    if cmd.allowed.contains(&player) {
                        return;
                    }
                    event.set_cancelled(true);
                    return;
                } else {
                    if cmd.allowed.contains(&player) {
                        return;
                    }
                    event.set_cancelled(true);
                    return;
                }
            }
        }
    }
}
