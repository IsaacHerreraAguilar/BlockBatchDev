all: build test

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

clean:
	cargo clean

update-snapshot:
	cargo test -- --update-snapshots

fmt:
	cargo fmt --all 