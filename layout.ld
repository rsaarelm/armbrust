OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)

/* ENTRY(start) */
ENTRY(ISR_VECTORS)

SECTIONS {
    . = 0x10000;

    /* Vector tables */
    .vector : ALIGN(4) {
        /* Reduces writes to NAND flash, which erases to 0xFF. */
        FILL(0xff)

        KEEP(*(.isr_vector))
        KEEP(*(.isr_vector_nvic))
    }/* > rom */

    /* Program code and read-only data. */
    .text : ALIGN(4) {
        FILL(0xff)
        *(.text*)
        *(.rodata .rodata.*)
    }/* > rom */

    .data : { *(.data) }
    .bss : { *(.bss) }
}

/* Guessing from ironkernel init asm... */
__STACK_BASE = 0x18000;
