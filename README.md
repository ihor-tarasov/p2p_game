# P2P Game

## Deps

```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo install matchbox_server
cargo install cargo-watch
```

## Run


### Terminal 1
```
matchbox_server
```

### Terminal 2
```
cargo run --release
```

### Terminal 2 in watch mode
```
cargo watch -cx "run --release"
```
