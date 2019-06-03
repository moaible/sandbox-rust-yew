setup:
	cargo install cargo-web --force
	rustup override set nightly
	make build
build:
	cargo web build --target wasm32-unknown-unknown
run: run-release
	
run-debug:
	cargo web start
run-release:
	cargo web start --release
