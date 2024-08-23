ENTRY(_start)

SECTIONS {
    . = 0xffffffff80000000; /* Kernel base address */

    .text : {
        *(.text)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }
}
