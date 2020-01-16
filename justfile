build:
  wasm-pack build --release --target=web

serve: build
  npx firebase serve

deploy: build
  npx firebase deploy --only hosting
