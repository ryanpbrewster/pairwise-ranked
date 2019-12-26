```
cargo install cargo-web
cargo web build --release

cd static/
ln -sf ../target/wasm32-unknown-unknown/release/stack-machine.wasm .
ln -sf ../target/wasm32-unknown-unknown/release/stack-machine.js .
./serve.py
```

then visit `localhost:8080`
