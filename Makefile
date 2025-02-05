debug_target := target/riscv64gc-unknown-none-elf/debug/rust_risc_v_cross_comp

run:
	cargo clean
	cargo run

build:
	cargo clean
	cargo build

release:
	cargo clean
	cargo build --release

gdb_server:
	qemu-system-riscv64 -machine virt -serial 'mon:stdio' -nographic -s -S -bios $(debug_target)

gdb_client:
	rust-gdb $(debug_target) -ex "target remote :1234" -ex "set print pretty on"
