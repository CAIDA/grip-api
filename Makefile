build:
	$(HOME)/.cargo/bin/cargo build --release

install: build
	sudo install -p --backup=none -v -m 0755 target/release/grip-api /usr/local/bin/grip-api
	sudo install -p --backup=none -v -m 0755 target/release/grip-cli /usr/local/bin/grip-cli
	sudo install -p --backup=none -v -m 0755 Rocket.toml /usr/local/etc/Rocket.toml

restart:
	sudo service grip-api restart

clean:
	cargo clean
