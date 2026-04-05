MEMORY
{
  FLASH : ORIGIN = 0x00004000, LENGTH = 0xC000
  RAM : ORIGIN = 0x20000000, LENGTH = 8K
}

_heap_size = 1K;
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Force the linker to keep the Toboot config symbol. */
EXTERN(__TOBOOT_CONFIG);

/* .text starts after vector table (0x94) + Toboot config (0x18). */
_stext = ORIGIN(FLASH) + 0x94 + 0x18;
