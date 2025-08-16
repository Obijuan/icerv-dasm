#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE andi
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum,    inst,  result, val1,   imm
#   TEST_IMM_OP( 2, andi, 0xff00ff00, 0xff00ff00, 0xf0f );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0xff00ff00

    #-- Resultado obtenido
    andi x14, x13, 0xffffff0f

    #-- x7: Valor del resultado esperado
    li x7, 0xff00ff00

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 3, andi, 0x000000f0, 0x0ff00ff0, 0x0f0 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x0ff00ff0

    #-- Resultado obtenido
    andi x14, x13, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0x000000f0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 4, andi, 0x0000000f, 0x00ff00ff, 0x70f );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x00ff00ff

    #-- Resultado obtenido
    andi x14, x13, 0x70f

    #-- x7: Valor del resultado esperado
    li x7, 0x0000000f

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_IMM_OP( 5, andi, 0x00000000, 0xf00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0xf00ff00f

    #-- Resultado obtenido
    andi x14, x13, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#   TEST_IMM_SRC1_EQ_DEST( 6, andi, 0x00000000, 0xff00ff00, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x11, 0xff00ff00

    #-- Resultado obtenido
    andi x11, x11, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst,    result,     val1,      imm 
#   TEST_IMM_DEST_BYPASS( 7,  0, andi, 0x00000700, 0x0ff00ff0, 0x70f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    li x4, 0

    #-- Valor 1
1:  li x1, 0x0ff00ff0

      #-- Valor inmediato
      andi x14, x1, 0x70f

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00000700
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
#   TEST_IMM_DEST_BYPASS( 8,  1, andi, 0x000000f0, 0x00ff00ff, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00ff00ff

      #-- Valor inmediato
      andi x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x000000f0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,  val1, imm 
#   TEST_IMM_DEST_BYPASS( 9,  2, andi, 0xf00ff00f, 0xf00ff00f, 0xf0f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    li x4, 0

    #-- Valor 1
1:  li x1, 0xf00ff00f

      #-- Valor inmediato
      andi x14, x1, 0xffffff0f

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xf00ff00f
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1,      imm 
#   TEST_IMM_SRC1_BYPASS( 10, 0, andi, 0x00000700, 0x0ff00ff0, 0x70f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    li x4, 0

    #-- Valor 1
1:  li x1, 0x0ff00ff0

      #-- Valor inmediato
      andi x14, x1, 0x70f

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00000700
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1,       imm 
#   TEST_IMM_SRC1_BYPASS( 11, 1, andi, 0x000000f0, 0x00ff00ff, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00ff00ff

      #-- Valor inmediato
      andi x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x000000f0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,        val1,     imm 
#   TEST_IMM_SRC1_BYPASS( 12, 2, andi, 0x0000000f, 0xf00ff00f, 0x70f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    li x4, 0

    #-- Valor 1
1:  li x1, 0xf00ff00f

      #-- Valor inmediato
      andi x14, x1, 0x70f

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x0000000f
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, result, imm
#   TEST_IMM_ZEROSRC1( 13, andi, 0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 13

    #-- imm
    andi x1, x0, 0x0f0

    #-- Result
    li  x7, 0
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 14, andi, 0x00ff00ff, 0x70f );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 14

    #-- val1
    li  x1, 0x00ff00ff

    #-- imm
    andi x0, x1, 0x70f

    li  x7, 0
    bne x0, x7, fail;


    #-- Test OK
    #-- x1=1
    #-- x3=Nº de test
pass:
    li x1, 1
    j .

    #-- Test NO pasado
    #-- x1=0
    #-- x3= Nº de test
fail:
    li x1, 0
    j .










