#![no_std]
#![no_main]

use core::panic::PanicInfo;

const UART_BASE: usize = 0x10000000;
const UART_THR: *mut u8 = UART_BASE as *mut u8;     // Transmit Holding Register
const UART_LSR: *mut u8 = (UART_BASE + 5) as *mut u8; // Line Status Register
const UART_LSR_EMPTY_MASK: u8 = 0x20;               // Transmitter Empty bit

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Wait until UART is ready to transmit
        while (*UART_LSR & UART_LSR_EMPTY_MASK) == 0 {}
        
        *UART_THR = b'B'; // Send 'A' over serial
    }
    loop {}
}