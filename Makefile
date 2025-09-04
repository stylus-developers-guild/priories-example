
priories.wasm: $(shell find src Cargo.toml Cargo.lock)
	@cargo build --release --target wasm32-unknown-unknown
	@cp -f target/wasm32-unknown-unknown/release/priories.wasm priories.wasm
