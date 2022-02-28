build:
	cargo build --release

install:
	cargo install --path .

uninstall:
	cargo uninstall hwt_ui

clean:
	cargo clean

release-mac:
	strip target/release/hwt
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/hwt-mac.tar.gz ./hwt

release-win:
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/hwt-win.tar.gz ./hwt.exe

release-linux:
	strip target/release/hwt
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/hwt-linux.tar.gz ./hwt