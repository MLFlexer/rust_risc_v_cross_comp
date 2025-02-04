#![no_std]
#![no_main]

// The following info is specific to the Qemu virt machine.
// The base address is 0x80000000, the UART address base is 0x10000000
// The UART is UART16550
// https://opensocdebug.readthedocs.io/en/latest/02_spec/07_modules/dem_uart/uartspec.html
const UART_BASE: usize = 0x10000000;
const UART_THR: *mut u8 = UART_BASE as *mut u8;     // Transmit Holding Register
const UART_LSR: *mut u8 = (UART_BASE + 5) as *mut u8; // Line Status Register
const UART_LSR_EMPTY_MASK: u8 = 0x20;               // Transmitter Empty bit
                                                    // we probably need to enable uart fifo as per https://www.youtube.com/watch?v=HC7b1SVXoKM
extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let hello_str = b"hello, world!";

    unsafe {
        for c in hello_str.iter() {
        while (*UART_LSR & UART_LSR_EMPTY_MASK) == 0 {}
        *UART_THR = *c;
            
        }
    }
    loop { }
}
