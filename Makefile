run:
	cargo run --target riscv64gc-unknown-linux-gnu

build:
	cargo build --target riscv64gc-unknown-linux-gnu

release:
	cargo build --release --target riscv64gc-unknown-linux-gnu
