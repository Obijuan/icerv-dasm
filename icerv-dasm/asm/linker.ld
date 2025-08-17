/*OUTPUT_ARCH(riscv)*/

SECTIONS
{
    . = 0x0;  /* Establece la dirección inicial en 0 */
    TEXT : { *(.text) }
    DATA : { *(.data) }
    
}
