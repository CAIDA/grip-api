APP_DIR	= /var/lib/bgphijacks-dashboard

build:
	$(HOME)/.cargo/bin/cargo build --release

clear:
	sudo rm -rf $(APP_DIR)/*

install: clear build
	sudo service bgphijacks-dashboard stop

	sudo install -p --backup=none -v -m 0755 target/release/hijacks_dashboard /usr/local/bin/bgphijacks-dashboard
	sudo cp -r app templates Rocket.toml $(APP_DIR)/

	sudo service bgphijacks-dashboard start

clean:
	cargo clean
