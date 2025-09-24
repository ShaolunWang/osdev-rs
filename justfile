build:
	cargo build && cargo bootimage
qemu:
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-osdev/debug/bootimage-osdev.bin
