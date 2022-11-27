
__rpi4_physical_dram_start_addr = 0;
__rpi4_physical_bin_load_addr = 0x80000;

ENTRY(__rpi4_physical_bin_load_addr)

/* FLAGS(6) == RW (readable and writeable section)
 * FLAGS(5) == RX (readable and executable section) */

PHDRS {
	segment_boot_core_stack	PT_LOAD	FLAGS(6);
	segment_code		PT_LOAD FLAGS(5);
	segment_data		PT_LOAD	FLAGS(6);
}

SECTIONS {
	. = __rpi4_physical_dram_start_addr;
	
	.boot_core_stack (NOLOAD) :
	{
		
		. += __rpi4_physical_bin_load_addr;

		__boot_core_stack_end = .;
	} :segment_boot_core_stack

	.text : {
		KEEP(*(.text._start))
		*(.text._rust_entry)
		*(.text*)
	} :segment_code
	
	.data : {
		*(.data*)
	} :segment_data

	.rodata : ALIGN(8) {
		*(rodata*)
	} :segment_code

	.bss (NOLOAD) : ALIGN(16) {
		__bss_start = .;
		*(.bss*)
		. = ALIGN (16)
		__bss_end = .;
	} :segment_data

	.got : { *(.got*) }
	ASSERT(SIZEOF(.got) == 0, "asd")

	/DISCARD/ : { *(.comment)* }
}
