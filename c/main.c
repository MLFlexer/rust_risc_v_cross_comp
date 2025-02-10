#include <stdint.h>

#define UART_BASE 0x10000000
#define UART_THR ((volatile uint8_t *)UART_BASE)     // Transmit Holding Register
#define UART_LSR ((volatile uint8_t *)(UART_BASE + 5)) // Line Status Register
#define UART_LSR_EMPTY_MASK 0x20                   // Transmitter Empty bit

extern void hello_from_rust();

void hello_from_c() {
    const char *message = "Hello from C!\n";

    while (*message) {
        while ((*UART_LSR & UART_LSR_EMPTY_MASK) == 0);
        *UART_THR = *message++;
    }
}

void main() {
  hello_from_c();
  hello_from_rust();
  while (1); // Dont return
}
