name := $(shell dasel -f Cargo.toml package.name)

.PHONY: dev debug release test clean

dev:
	while true; do fd . | entr -ccd make test debug; done

debug:
	mkdir -p dist
	cargo build --target x86_64-unknown-linux-musl
	ln -f "target/x86_64-unknown-linux-musl/debug/${name}" "dist/"

release:
	mkdir -p dist
	cargo build --release --target x86_64-unknown-linux-musl
	ln -f "target/x86_64-unknown-linux-musl/release/${name}" "dist/"

test:
	cargo test -- --nocapture

clean:
	cargo clean
