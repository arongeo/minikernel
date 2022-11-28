__rpi_physical_dram_load_addr = 0;
__rpi_physical_bin_load_addr = 0x80000;

ENTRY(__rpi_physical_bin_load_addr)

PHDRS 
{
	segment_boot_core_stack	PT_LOAD	FLAGS(6);
	segment_code		PT_LOAD FLAGS(5);
	segment_data		PT_LOAD	FLAGS(6);
}

SECTIONS 
{
	. = __rpi_physical_dram_load_addr;

	.boot_core_stack (NOLOAD) :
	{
		. += __rpi_physical_bin_load_addr;

		__boot_core_stack_end = .;
	} :segment_boot_core_stack

	.text : {
		KEEP(*(.text._start))
		*(.text._start_args)
		*(.text._rust_entry)
		*(.text*)
	} :segment_code

	.rodata : ALIGN(8) {
		*(rodata*)
	} :segment_code
	
	.data : {
		*(.data*)
	} :segment_data

	.bss (NOLOAD) : ALIGN(16) {
		__bss_start = .;
		*(.bss*);
		. = ALIGN(16);
		__bss_end = .;
	} :segment_data

}
