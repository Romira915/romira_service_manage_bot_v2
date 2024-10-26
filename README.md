# romira_service_manage_bot_v2

## Description

This is a telegram bot that allows you to manage the services of a server. It is a bot that allows you to start, stop,
restart and check the status of the services of a server.

## development

### Install the necessary tools

```shell
cargo install cargo-watch systemfd
```

### Run the project

#### discord_bod_client

```shell
cargo watch -x "run -p discord_bot_client"
```

#### wakaba_service_manage_bot_v2

```shell
systemfd --no-pid -s http::8080 -- cargo watch -x "run -p wakaba_game_service_manage_api"
```
