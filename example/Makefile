
all: deploy

kernel-hash.o: src/kernel.rs
	cargo rustc --release -v --target arm-unknown-linux-gnueabihf -- --emit=obj -o build/kernel.o

kernel.o: kernel-hash.o
	cp build/`ls -t build | grep -E "kernel-.*\.o" | head -n1` build/kernel.o
	cp target/arm-unknown-linux-gnueabihf/release/deps/`ls -t target/arm-unknown-linux-gnueabihf/release/deps | grep -E "librusty_metal_raspberry_pi-.*\.rlib" | head -n1` build/librusty_metal_raspberry_pi.rlib

kernel.elf: kernel.o
	arm-none-eabi-gcc -T linker.ld -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles build/kernel.o build/librusty_metal_raspberry_pi.rlib -o build/kernel.elf

kernel.img: kernel.elf
	arm-none-eabi-objcopy build/kernel.elf -O binary build/kernel.img

deploy: kernel.img
	cp build/kernel.img /home/tim/web/pi/kernel.img
	cp build/kernel.img /home/tim/web/pi/kernel.img.a
