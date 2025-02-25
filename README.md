# rhkmd-games

## setup
Install rustup
``` 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install wasm stuff
```
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
```

## run one of the test games in their directory
```
cargo run --target wasm32-unknown-unknown
```

or specify ip
```
WASM_SERVER_RUNNER_ADDRESS=1.2.3.4 cargo run --target wasm32-unknown-unknown
```