/* target arch */
OUTPUT_ARCH(riscv)

/* execute entry */
ENTRY(_start)

/* data storage start address */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . this address(location counter) */
    . = BASE_ADDRESS;

    /* start indicates all starting positions */
    kernel_start = .;

    text_start = .;
    
    /* .text field */
    .text : {
        /* put entry at the top */
        *(.text.entry)
        /* the .text field to be linked are concentrated here */
        *(.text .text.*)
    }

    rodata_start = .;

    /* .rodata field */
    .rodata : {
        /* the .rodata field to be linked are concentrated here */
        *(.rodata .rodata.*)
    }

    data_start = .;
 
    /* .data field */
    .data : {
        /* the .data field to be linked are concentrated here */
        *(.data .data.*)
    }

    bss_start = .;
 
    /* .bss field */
    .bss : {
        /* the .bss field to be linked are concentrated here */
        *(.sbss .bss .bss.*)
    }

    /* end address */
    kernel_end = .;
}
