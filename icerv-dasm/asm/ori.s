#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE ORI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum,    inst,  result, val1,   imm
#   TEST_IMM_OP( 2, ori, 0xffffff0f, 0xff00ff00, 0xf0f );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0xff00ff00

    #-- Resultado obtenido
    ori x14, x13, 0xffffff0f

    #-- x7: Valor del resultado esperado
    li x7, 0xffffff0f

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 3, ori, 0x0ff00ff0, 0x0ff00ff0, 0x0f0 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x0ff00ff0

    #-- Resultado obtenido
    ori x14, x13, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0x0ff00ff0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 4, ori, 0x00ff07ff, 00ff00ff, 0x70f );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x00ff00ff

    #-- Resultado obtenido
    ori x14, x13, 0x70f

    #-- x7: Valor del resultado esperado
    li x7, 0x00ff07ff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_IMM_OP( 5, ori, 0xf00ff0ff, 0xf00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0xf00ff00f

    #-- Resultado obtenido
    ori x14, x13, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0xf00ff0ff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#   TEST_IMM_SRC1_EQ_DEST( 6, ori, 0xff00fff0, 0xff00ff00, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x11, 0xff00ff00

    #-- Resultado obtenido
    ori x11, x11, 0x0f0

    #-- x7: Valor del resultado esperado
    li x7, 0xff00fff0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst,    result,     val1,      imm 
#   TEST_IMM_DEST_BYPASS( 7,  0, ori, 0x0ff00ff0, 0x0ff00ff0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    li x4, 0

    #-- Valor 1
1:  li x1, 0x0ff00ff0

      #-- Valor inmediato
      ori x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x0ff00ff0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
#   TEST_IMM_DEST_BYPASS( 8,  1, ori, 0x00ff07ff, 0x00ff00ff, 0x70f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00ff00ff

      #-- Valor inmediato
      ori x14, x1, 0x70f

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00ff07ff
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,  val1, imm 
#   TEST_IMM_DEST_BYPASS( 9,  2, ori, 0xf00ff0ff, 0xf00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    li x4, 0

    #-- Valor 1
1:  li x1, 0xf00ff00f

      #-- Valor inmediato
      ori x14, x1, 0x0f0

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
#   TEST_IMM_SRC1_BYPASS( 10, 0, ori, 0x0ff00ff0, 0x0ff00ff0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    li x4, 0

    #-- Valor 1
1:  li x1, 0x0ff00ff0

      #-- Valor inmediato
      ori x14, x1, 0x0f0

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x0ff00ff0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1,       imm 
#   TEST_IMM_SRC1_BYPASS( 11, 1, ori, 0xffffffff, 0x00ff00ff, 0xf0f );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00ff00ff

      #-- Valor inmediato
      ori x14, x1, 0xffffff0f

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xffffffff
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,        val1,     imm 
#   TEST_IMM_SRC1_BYPASS( 12, 2, ori, 0xf00ff0ff, 0xf00ff00f, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    li x4, 0

    #-- Valor 1
1:  li x1, 0xf00ff00f

      #-- Valor inmediato
      ori x14, x1, 0x0f0

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
#                 testnum, inst, result, imm
#   TEST_IMM_ZEROSRC1( 13, ori, 0x0f0, 0x0f0 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 13

    #-- imm
    ori x1, x0, 0x0f0

    #-- Result
    li  x7, 0x0f0
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 14, ori, 0x00ff00ff, 0x70f );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 14

    #-- val1
    li  x1, 0x00ff00ff

    #-- imm
    ori x0, x1, 0x70f

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







