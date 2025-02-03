run:
	cargo run --target riscv64gc-unknown-none-elf

build:
	cargo build --target riscv64gc-unknown-none-elf

release:
	cargo build --release --target riscv64gc-unknown-none-elf
