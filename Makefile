
run:
	cargo clean
	cargo run --target riscv64gc-unknown-none-elf

build:
	cargo clean
	cargo build --target riscv64gc-unknown-none-elf

release:
	cargo clean
	cargo build --release --target riscv64gc-unknown-none-elf
