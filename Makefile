setup-rust:
	curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2020-01-01 -y

build: setup-rust
	$(HOME)/.cargo/bin/cargo build --release

install: build
	sudo service grip-api stop
	sudo install -p --backup=none -v -m 0755 target/release/grip-api /usr/local/bin/grip-api
	sudo install -p --backup=none -v -m 0755 target/release/grip-cli /usr/local/bin/grip-cli
	sudo service grip-api start

clean:
	cargo clean
