# Rust Chat Server Tutorial



## Starting the client app
The example app is a rust app written with [egui](https://github.com/emilk/egui).
### Native app
To start the native app, run 
```bash
cargo run -p app
```
### Web app
First, install trunk and add the wasm32 target. Trunk is a bundler for rust WASM web apps.
```bash
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```
Then, run the app by calling 
```bash
cd app
trunk serve
```
