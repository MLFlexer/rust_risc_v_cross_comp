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

#[cfg(not(debug_assertions))]
extern crate panic_halt;

#[cfg(debug_assertions)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    hprintln!("{}", info);
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

use riscv_semihosting::{debug, hprintln};
use riscv_rt::entry;

extern "C" {
    fn hello_from_c() -> ();
}

fn write_c(c: u8) {
    unsafe {
        while (*UART_LSR & UART_LSR_EMPTY_MASK) == 0 {}
        *UART_THR = c;
    }
}

use heapless::Vec;

#[entry]
fn main() -> ! {
    let hello_str = b"hello, world!\n";

    for c in hello_str.iter() {
        write_c(*c);
    }

    unsafe { hello_from_c(); }

    let mut xs: Vec<u8, 8> = Vec::new();

    hprintln!("xs: {:?}", xs);
    xs.push(42);
    hprintln!("xs: {:?}", xs);
    xs.push(69);
    hprintln!("xs: {:?}", xs);
    xs.pop();
    hprintln!("xs: {:?}", xs);


    hprintln!("Hello Semihosting World!");
    debug::exit(debug::EXIT_SUCCESS);
    loop { }
}

#[cfg(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 3), 5);
    }
}
