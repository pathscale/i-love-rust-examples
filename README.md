# i-love-rust-examples
This is a async websocket server template, with nice codegen included. Every tool required from Data Access Layer to backend endpoints are generated without user intervention.

This project is a micro service implementation. 

## How to build

```shell
cargo build
cargo build --release
```
You can find the binary in `target/debug/` or `target/release/` folder.

## How to run

```shell
cargo run --bin auth
cargo run --bin admin
cargo run --bin user
```

## Structure explained

`src/codegen` core codegen logic
`src/gen` codegen target
`src/lib` common code
`src/service` implementation of services
`src/service/{srv}/main.rs` main entry of service
`src/service/{srv}/endpoints.rs` declaration of endpoints
`src/service/{srv}/pg_func.rs` declaration of postgres procedural endpoints (DALs)
`src/service/{srv}/method.rs` implementation of endpoints
`tests` integration tests
`benches` benchmarks
`docs` documentation
`db` database related files
`etc` configuration files
`scripts` helper scripts


