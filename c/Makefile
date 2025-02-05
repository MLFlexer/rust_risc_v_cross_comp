default: main.elf

main.o: main.c
	riscv64-none-elf-gcc -c -g -O0 -mcmodel=medany -march=rv64id -mabi=lp64d -ffreestanding -o main.o main.c

start.o: start.s
	riscv64-none-elf-as -march=rv64id -o start.o start.s

main.elf: main.o start.o link.ld
	riscv64-none-elf-ld -T link.ld -m elf64lriscv -o main.elf main.o start.o ./target/riscv64gc-unknown-none-elf/debug/libc.a

run:
	qemu-system-riscv64 -machine virt -serial 'mon:stdio' -nographic -bios ./main.elf

clean:
	rm ./main.o
	rm ./start.o
	rm ./main.elf
	cargo clean
