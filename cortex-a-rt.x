/* # Developer notes

- Symbols that start with a double underscore (__) are considered "private"

- Symbols that start with a single underscore (_) are considered "semi-public"; they can be
  overridden in a user linker script, but should not be referred from user code (e.g. `extern "C" {
  static mut __sbss }`).

- `EXTERN` forces the linker to keep a symbol in the final binary. We use this to make sure a
  symbol if not dropped if it appears in or near the front of the linker arguments and "it's not
  needed" by any of the preceding objects (linker arguments)

- `PROVIDE` is used to provide default values that can be overridden by a user linker script

- On alignment: it's important for correctness that the VMA boundaries of both .bss and .data *and*
  the LMA of .data are all 4-byte aligned. These alignments are assumed by the RAM initialization
  routine. There's also a second benefit: 4-byte aligned boundaries means that you won't see
  "Address (..) is out of bounds" in the disassembly produced by `objdump`.
*/

/* Provides information about the memory layout of the device */
/* This will be provided by the user (see `memory.x`) or by a Board Support Crate */
INCLUDE memory.x
INCLUDE stack.x

/* # Entry point = reset vector */
EXTERN(__cortex_a_rt_reset);
ENTRY(__cortex_a_rt_reset);

/* # Sections */
SECTIONS
{
  /* PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM)); */

  /* ## Sections in FLASH */
  /* ### Vector table */
  .vectors load_address :
  {
    /* Reset vector */
    KEEP(*(.init0)); /* this is the `__RESET_VECTOR` symbol */
  }

  PROVIDE(_stext = ADDR(.vectors) + SIZEOF(.vectors));

  /* ### .text */
  .text _stext :
  {
    __stext = .;
    *(.init1);
    *(.except)

    *(.text .text.*);

    . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
    __etext = .;
  }

  /* ### .rodata */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
    __erodata = .;
  }

  /* ## Sections in RAM */
  /* ### .data */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  }
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .data` to
   * use the .data loading mechanism by pushing __edata. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __edata = .;

  /* ### .bss */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON); /* Uninitialized C statics */
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  }
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .bss` to
   * use the .bss zeroing mechanism by pushing __ebss. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __ebss = .;

  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  }

  /* Place the heap right after `.uninit` in RAM */
  PROVIDE(__sheap = __euninit);

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* Do not exceed this mark in the error messages below                                    | */
/* # Alignment checks */
ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(cortex-a-rt): .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(cortex-a-rt): .bss is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
BUG(cortex-a-rt): start of .heap is not 4-byte aligned");

/* # Position and size checks */

/* ## .vectors */
ASSERT(ADDR(.vectors) == load_address, "
BUG(cortex-a-rt): vectors section are not at the program load address");

ASSERT(ADDR(.vectors) == __cortex_a_rt_reset, "
BUG(cortex-a-rt): __cortex_a_rt_reset is not at the start of .vectors section");

ASSERT(SIZEOF(.vectors) == 32, "
BUG(cortex-a-rt): vectors section has invalid size");

/* ## .text */

/* ASSERT(_stext == __cortex_a_init, "
BUG(cortex-a-rt): __cortex_a_init is not at the start of .text section"); */

/* # Other checks */
ASSERT(SIZEOF(.got) == 0, "
ERROR(cortex-a-rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");
/* Do not exceed this mark in the error messages above                                    | */
