__sp = 0x10000;

SECTIONS
{
    . =0x00000000;
    .text.init : {
        *(.text.init)
    }

    .text : {
        *(.text)
    }

    __global_pointer$ = .  + 0x800;
    .data : {
        *(.sbss)
        *(COMMON)
        *(.bss)

        *(.data*)
        *(.sdata)
	*(.srodata*)
         *(.rodata)
        *(.got*)
	/**(._edata*)*/
	*(._end)
    	/*_edata=.;*/
	 _edata = .; PROVIDE (edata = .);
 	. = .;
    }

    /*_edata = .; PROVIDE (edata = .);*/
    _end=.;
}
