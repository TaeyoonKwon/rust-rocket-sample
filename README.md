# rust-rocket-sample
[![CI](https://github.com/TaeyoonKwon/rust-rocket-sample/actions/workflows/ci.yaml/badge.svg)](https://github.com/TaeyoonKwon/rust-rocket-sample/actions/workflows/ci.yaml)

Fully working CRUD REST API example using 
- Rust (stable)
- Rocket.rs
- mongodb
- okapi


## 🔧 Building and Testing

### debug mode
> cargo run

### release mode
> cargo build --release && cargo run --release


### unit testing
> cargo test

<br/>

ℹ️ _You should create your own `.env` file including `MONGO_URI`, `MONGO_DB_NAME`, and `API_KEY` to run it._

## 📑 License
[MIT](https://github.com/TaeyoonKwon/rust-rocket-sample/blob/main/LICENSE) Copyright (c) 2022 Taeyoon Kwon
