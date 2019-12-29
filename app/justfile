build:
  wasm-pack build --release --target=web

serve: build
  fb serve

deploy: build
  fb deploy --only hosting
