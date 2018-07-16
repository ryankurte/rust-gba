/*
 * Gameboy Advanced ROM Linker Script
 * https://github.com/ryankurte/rust-gba
*/

OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)

MEMORY
{
  ROM    : ORIGIN = 0x08000000, LENGTH = 32M
  EWRAM	 : ORIGIN = 0x02000000, LENGTH = 256K
  IWRAM	 : ORIGIN = 0x03000000, LENGTH = 32K
}

__text_start	=	ORIGIN(ROM);
__app_start   = __text_start + 0xe0;
__eheap_end	=	ORIGIN(EWRAM) + LENGTH(EWRAM);
__iwram_start	=	ORIGIN(IWRAM);
__iwram_top	=	ORIGIN(IWRAM) + LENGTH(IWRAM);;
__sp_irq	=	__iwram_top - 0x060;
__sp_usr	=	__sp_irq - 0x0a0;
__irq_flags	=	0x03007ff8;

SECTIONS
{
  . = __text_start;

  /* static data stored in cart ROM */
  .header :
  {
    KEEP(*(.header.header));
  } > ROM

  .text : { 
    KEEP(*(.text.boot));
    KEEP(*(.text.reset_handler));
    KEEP(*(.text.isr));
    *(.text .text.*); 
    . = ALIGN(4); 
  } > ROM

  .rodata : { 
    *(.rodata .rodata.*); 
    . = ALIGN(4); 
  } > ROM

  /* dynamic data placed in IWRAM */
  .data : { 
    _sidata = LOADADDR(.data);
    . = ALIGN(4);
    _sdata = .;

    *(.data);

    . = ALIGN(4);
     _edata = .; 
  } > IWRAM AT > ROM

  .bss : {
    . = ALIGN(4);
    _sbss = .;

    *(.bss); 

    . = ALIGN(4); 
    _ebss = .;  
  } > IWRAM

  /* discard arm symbols */
  /DISCARD/ : { *(.ARM.exidx.*); }
}
