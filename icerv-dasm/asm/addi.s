#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE ADDI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 2
#  TEST_IMM_OP( 2,  addi, 0x00000000, 0x00000000, 0x000 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x00

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 3
#         testnum,  inst,  result,       val1,     imm
#  TEST_IMM_OP( 3,  addi, 0x00000002, 0x00000001, 0x001 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0x00000002

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 4
#         testnum,  inst,  result,       val1,     imm
#  TEST_IMM_OP( 4,  addi, 0x0000000a, 0x00000003, 0x007 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x00000003

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x007

    #-- x7: Valor del resultado esperado
    li x7, 0x0000000a

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 5
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 5,  addi, 0xfffffffffffff800, 0x0000000000000000, 0x800 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0xFFFFF800

    #-- x7: Valor del resultado esperado
    li x7, 0xFFFFF800

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 6
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 6,  addi, 0xffffffff80000000, 0xffffffff80000000, 0x000 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x000

    #-- x7: Valor del resultado esperado
    li x7, 0x80000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 7
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 7,  addi, 0xffffffff7ffff800, 0xffffffff80000000, 0x800 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0xFFFFF800

    #-- x7: Valor del resultado esperado
    li x7, 0x7FFFF800

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 8
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 8,  addi, 0x00000000000007ff, 0x00000000, 0x7ff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x7ff

    #-- x7: Valor del resultado esperado
    li x7, 0x000007ff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 9
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 9,  addi, 0x000000007fffffff, 0x7fffffff, 0x000 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x000

    #-- x7: Valor del resultado esperado
    li x7, 0x7fffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 10
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 10, addi, 0x00000000800007fe, 0x7fffffff, 0x7ff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x7ff

    #-- x7: Valor del resultado esperado
    li x7, 0x800007fe

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 11
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 11, addi, 0xffffffff800007ff, 0xffffffff80000000, 0x7ff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x7ff

    #-- x7: Valor del resultado esperado
    li x7, 0x800007ff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 12
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 12, addi, 0x000000007ffff7ff, 0x000000007fffffff, 0x800 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0xFFFFF800

    #-- x7: Valor del resultado esperado
    li x7, 0x7ffff7ff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 13
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 13, addi, 0xffffffffffffffff, 0x0000000000000000, 0xfff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 13

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0xffffffff

    #-- x7: Valor del resultado esperado
    li x7, 0xffffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 14
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 14, addi, 0x0000000000000000, 0xffffffffffffffff, 0x001 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 14

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 15
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 15, addi, 0xfffffffffffffffe, 0xffffffffffffffff, 0xfff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 15

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0xffffffff

    #-- x7: Valor del resultado esperado
    li x7, 0xfffffffe

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 16
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 16, addi, 0x0000000080000000, 0x7fffffff, 0x001 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 16

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0x80000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

  

  #-------------------------------------------------------------
  # Source/Destination tests
  #-------------------------------------------------------------

#  TEST_IMM_SRC1_EQ_DEST( 17, addi, 24, 13, 11 );

  #-------------------------------------------------------------
  # Bypassing tests
  #-------------------------------------------------------------

#  TEST_IMM_DEST_BYPASS( 18, 0, addi, 24, 13, 11 );
#  TEST_IMM_DEST_BYPASS( 19, 1, addi, 23, 13, 10 );
#  TEST_IMM_DEST_BYPASS( 20, 2, addi, 22, 13,  9 );
#
#  TEST_IMM_SRC1_BYPASS( 21, 0, addi, 24, 13, 11 );
#  TEST_IMM_SRC1_BYPASS( 22, 1, addi, 23, 13, 10 );
#  TEST_IMM_SRC1_BYPASS( 23, 2, addi, 22, 13,  9 );
#
#  TEST_IMM_ZEROSRC1( 24, addi, 32, 32 );
#  TEST_IMM_ZERODEST( 25, addi, 33, 50 );
#




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



#-- Pruebas para addi
#           testnum,  inst,  result,       val1,     imm
#    TEST_IMM_OP( 2,  addi, 0x00000000, 0x00000000, 0x000 );
#   x14 = val1 + imm --> x14 = result?


