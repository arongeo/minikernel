all:
	if [ -d "target/" ]; then rm -rd target/; fi
	rm -rf kernel8.img
	cargo build
	if [ -d "target/aarch64-unknown-none" ]; then aarch64-unknown-none-objcopy -O binary target/aarch64-unknown-none/debug/kernel4rpi kernel8.img; fi
