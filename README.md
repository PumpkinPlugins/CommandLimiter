# Command Limiter
[![Build Status](https://ci.vypal.me/job/PumpkinPlugins/job/CommandLimiter/job/master/badge/icon)](https://ci.vypal.me/job/PumpkinPlugins/job/CommandLimiter/job/master/)

A simple [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) plugin that allows admins to limit access to certain commands to certain players.

## Installing
[PumpkinPlugins](https://github.com/PumpkinPlugins) provides prebuilt plugin binaries for the most common platforms and architectures. If your host is among these, it is enough to download the appropriate binary and place it in the `plugins` directory of your Pumpkin server.

| Platform | Architecture | Download |
| -------- | ------------ | -------- |
| Linux    | amd64        | [Download](https://ci.vypal.me/job/PumpkinPlugins/job/CommandLimiter/job/master/lastSuccessfulBuild/artifact/artifacts/libcommandlimiter_x86_64_linux.so) |
| Linux    | arm64        | [Download](https://ci.vypal.me/job/PumpkinPlugins/job/CommandLimiter/job/master/lastSuccessfulBuild/artifact/artifacts/libcommandlimiter_aarch64_linux.so) |
| Windows  | amd64        | [Download](https://ci.vypal.me/job/PumpkinPlugins/job/CommandLimiter/job/master/lastSuccessfulBuild/artifact/artifacts/commandlimiter_x86_64_windows.dll) |

If your host is not among these, you will need to build the plugin yourself. To do this, you will need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system. Once you have Rust installed, you can clone this repository and run `cargo build --release` in the root directory of the repository. The compiled binary will be located at `target/release/libcommand_limiter.so` on Linux,  `target/release/command_limiter.dll` on Windows, and `target/release/libcommand_limiter.dylib` on macOS.

## Configuration
The plugin configuration is stored in the `config.json` file in the `plugins/command_limiter` directory. The configuration file is created automatically when the plugin is loaded for the first time. The configuration follows the JSON format and has the following structure:

```json
{
  "commands": [
    {
      "command": "command_name",
      "blacklist": false,
      "allowed": [
        "player1",
        "player2"
      ]
    }
  ],
  "block_message": "You are not allowed to use this command."
}
```

The configuration can also be modified in-game using commands.

## Usage
The plugin provides the following commands:
| Command | Aliases | Description | Permission |
| ------- | ------- | ----------- | ---------- |
| `/limitcommand <command> (blacklist) (players)` | `/limit, /lc` | Limits access to a command to certain players. | `OP level 4` |

The `/limitcommand` command can either be executed just with the command name, which will automatically blacklist everyone from using it, or with all three arguments. The `blacklist` argument is a boolean that determines whether the players listed in the `players` argument should be blacklisted or whitelisted. The `players` argument is a list of player names separated by commas.
