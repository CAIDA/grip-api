build:
	$(HOME)/.cargo/bin/cargo build --release

install: build
	sudo install -p --backup=none -v -m 0755 target/release/grip-api /usr/local/bin/grip-api
	sudo install -p --backup=none -v -m 0755 target/release/grip-cli /usr/local/bin/grip-cli

restart:
	sudo service grip-api restart

clean:
	cargo clean
