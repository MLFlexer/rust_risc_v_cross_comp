SECTIONS
{
  . = 0x80000000; /* Qemu virt machine load address for bios */
  .text : {
    KEEP(*(.text._start)); /* _start is forced to be keept at the top */
    *(.text*); /* all other code is placed after _start */
  }
  . = ALIGN (CONSTANT (COMMONPAGESIZE)); /* alligns .data to page size as to not place .data section in .text */
  .rodata : {
    *(.rodata*)
  }
  . = ALIGN (CONSTANT (COMMONPAGESIZE)); /* alligns .data to page size as to not place .data section in .text */
  .data : {
    *(.data*) /* stack */
  }
}
