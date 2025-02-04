OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80000000;
    .text : {
        KEEP(*(.text._start));
        *(.text*);
    }
    . = ALIGN (CONSTANT (COMMONPAGESIZE));
    .data : {
        *(.data*);
    }
    . = ALIGN (CONSTANT (COMMONPAGESIZE));
    .rodata : {
        *(.rodata*)
    }
}
