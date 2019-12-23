MEMORY
{
  /* 256 pages of 4 KiB each = 1MiB total space in FLASH.
     Allocate only a part of it for now */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  /* 0x2000_0000 is "Data Ram" accessed via the "System Bus"
     0x0080_0000 is "Code Ram" accessed via "DCODE" and "ICODE" buses */
  /* RAM 0 to RAM 7 (RAM 8 excluded), end = 0x2001_0000 */
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/
