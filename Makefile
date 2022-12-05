PERI_MODE = "low"

all:
	if [ -d "target/" ]; then rm -rd target/; fi
	rm -rf kernel8.img
	cargo rustc -- --cfg peri_mode='$(PERI_MODE)' -C link-arg=--script=linker.ld 
	if [ -d "target/aarch64-unknown-none" ]; then aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/debug/kernel4rpi kernel8.img; fi
