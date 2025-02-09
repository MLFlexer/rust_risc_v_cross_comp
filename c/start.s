# initalize stack
# jump to main

.section .text._start # _start is the "main" of risc-v embedded
.global _start
_start:
  la sp, __stack_top # load address (la) of the stack (__stack_top) into the stack pointer (sp)
  add s0, sp, zero # set the frame pointer to the top of the stack (sp) by setting s0 = sp + zero -> s0 = __stack_top + 0
  jal zero, main # Jump and link (jal)
loop: j loop # loop forever if we return from main()

# Initialize the stack
.section .data
.space 1024*8 # allocates 8K memory for the stack
.align 16 # alligns it by 16 bytes
__stack_top: # stack grows down, so this is the top
