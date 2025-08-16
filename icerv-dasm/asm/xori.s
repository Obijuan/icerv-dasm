#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SLTI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum,    inst,  result, val1,   imm
# TEST_IMM_OP( 2, xori, 0xffffffffff00f00f, 0x0000000000ff0f00, 0xf0f );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido
    xori x14, x13, 0x00

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 3, xori, 0x000000000ff00f00, 0x000000000ff00ff0, 0x0f0 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    xori x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 4, xori, 0x00ff0ff0, 0x00ff08ff, 0x70f );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x00ff08ff

    #-- Resultado obtenido
    xori x14, x13, 0x70f

    #-- x7: Valor del resultado esperado
    li x7, 0x00ff0ff0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#  TEST_IMM_OP( 5, xori, 0xf00ff0ff, 0xf00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0xf00ff00f

    #-- Resultado obtenido
    xori x14, x13, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0xf00ff0ff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#   TEST_IMM_SRC1_EQ_DEST( 6, xori, 0xff00f00f, 0xff00f700, 0x70f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x11, 0xff00f700

    #-- Resultado obtenido
    xori x11, x11, 0x70f

    #-- x7: Valor del resultado esperado
    li x7, 0xff00f00f

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst,    result,     val1,      imm 
#   TEST_IMM_DEST_BYPASS( 7,  0, xori, 0x0ff00f00, 0x0ff00ff0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    li x4, 0

    #-- Valor 1
1:  li x1, 0x0ff00ff0

      #-- Valor inmediato
      xori x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x0ff00f00
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
# TEST_IMM_DEST_BYPASS( 8,  1, xori, 0x00ff0ff0, 0x00ff08ff, 0x70f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00ff08ff

      #-- Valor inmediato
      xori x14, x1, 0x70f

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00ff0ff0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,  val1, imm 
#  TEST_IMM_DEST_BYPASS( 9,  2, xori, 0xf00ff0ff, 0xf00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    li x4, 0

    #-- Valor 1
1:  li x1, 0xf00ff00f

      #-- Valor inmediato
      xori x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xf00ff0ff
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1,      imm 
# TEST_IMM_SRC1_BYPASS( 10, 0, xori, 0x0ff00f00, 0x0ff00ff0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    li x4, 0

    #-- Valor 1
1:  li x1, 0x0ff00ff0

      #-- Valor inmediato
      xori x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x0ff00f00
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1,       imm 
#  TEST_IMM_SRC1_BYPASS( 11, 1, xori, 0x00ff0ff0, 0x00ff0fff, 0x00f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00ff0fff

      #-- Valor inmediato
      xori x14, x1, 0x00f

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00ff0ff0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 12, 2, xori, 0xfffffffff00ff0ff, 0xfffffffff00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    li x4, 0

    #-- Valor 1
1:  li x1, 12

      #-- Valor inmediato
      xori x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 1
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, result, imm
#  TEST_IMM_ZEROSRC1( 13, xori, 0x0f0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 13

    #-- imm
    xori x1, x0, 0xffffffff

    #-- Result
    li  x7, 1
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 14, xori, 0x00ff00ff, 0x70f );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 14

    #-- val1
    li  x1, 0x00ff00ff

    #-- imm
    xori x0, x1, 0xffffffff

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


