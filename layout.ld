ENTRY(_reset);

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM   : ORIGIN = 0x20000000, LENGTH = 8K
}

SECTIONS
{
  .text :
  {
    /* Vector table */
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    LONG(_reset + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);
    LONG(_hang + 1);

    /* Reset handler */
    _reset = .;
    *(.text._reset)
    _hang = .;
    *(.text._hang)

    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
