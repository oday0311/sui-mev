# Sui MEV Bot


## Run 
Start the bot with your private key.

```bash
cargo run -r --bin arb start-bot -- --private-key ca7816d1cbf87e006f91f84ac236b290c31b2e5daf6b2d6a3319f9df55d242b4
```

## Supports

- BlueMove
- FlowX
- Aftermath
- Cetus 
- Kriya
- Abex
- Navi
- Turbos
- Deepbook
- Shio

## Relay
If you have a validator, you can let the validator push mempool transactions to your relay, which then send to the bot.

```bash
cargo run -r --bin relay
```