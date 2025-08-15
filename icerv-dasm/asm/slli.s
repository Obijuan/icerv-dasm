#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SLLI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#            testnum,  inst,  result,           val1,           imm
# TEST_IMM_OP( 2,  slli, 0x0000000000000001, 0x0000000000000001, 0  );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    slli x14, x13, 0x00

    #-- x7: Valor del resultado esperado
    li x7, 0x00000001

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#  TEST_IMM_OP( 3,  slli, 0x0000000000000002, 0x0000000000000001, 1  );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    slli x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0x00000002

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 4,  slli, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    slli x14, x13, 0x007

    #-- x7: Valor del resultado esperado
    li x7, 0x00000080

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#  TEST_IMM_OP( 5,  slli, 0x00004000, 0x00000001, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    slli x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0x4000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_IMM_OP( 6,  slli, 0x80000000, 0x00000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    slli x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x80000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_IMM_OP( 7,  slli, 0xffffffff, 0xffffffff, 0  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    slli x14, x13, 0

    #-- x7: Valor del resultado esperado
    li x7, 0xffffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#  TEST_IMM_OP( 8,  slli, 0xfffffffe, 0xffffffff, 1  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    slli x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0xfffffffe

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 9,  slli, 0xffffff80, 0xffffffff, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    slli x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0xffffff80

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

    #-- 🚧 DEBUG 🚧
    jal x0, pass
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

  
#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 17
#                  testnum,  inst,  result, val1, imm
#  TEST_IMM_SRC1_EQ_DEST( 17, addi, 24, 13, 11 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 17

    #-- Valor 1
    li x11, 13

    #-- Resultado obtenido: Sumar valor 1  y valor inmediato
    addi x11, x11, 11

    #-- x7: Valor del resultado esperado
    li x7, 24

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 18
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_DEST_BYPASS( 18, 0, addi, 24, 13, 11 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 18

    li x4, 0

    #-- Valor 1
1:  li x1, 13

      #-- Valor inmediato
      addi x14, x1, 11

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 24
    bne x6, x7, fail


#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 19
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_DEST_BYPASS( 19, 1, addi, 23, 13, 10 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 19

    li x4, 0

    #-- Valor 1
1:  li x1, 13

      #-- Valor inmediato
      addi x14, x1, 10

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 23
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 20
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_DEST_BYPASS( 20, 2, addi, 22, 13,  9 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 20

    li x4, 0

    #-- Valor 1
1:  li x1, 13

      #-- Valor inmediato
      addi x14, x1, 9

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 22
    bne x6, x7, fail
 
#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 21
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 21, 0, addi, 24, 13, 11 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 21

    li x4, 0

    #-- Valor 1
1:  li x1, 13

      #-- Valor inmediato
      addi x14, x1, 11

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 24
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 22
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 22, 1, addi, 23, 13, 10 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 22

    li x4, 0

    #-- Valor 1
1:  li x1, 13

      #-- Valor inmediato
      addi x14, x1, 10

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 23
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 23
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 23, 2, addi, 22, 13,  9 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 23

    li x4, 0

    #-- Valor 1
1:  li x1, 13

      #-- Valor inmediato
      addi x14, x1, 9

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 22
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 24
#                 testnum, inst, result, imm
#  TEST_IMM_ZEROSRC1( 24, addi, 32, 32 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 24

    #-- imm
    addi x1, x0, 32

    #-- Result
    li  x7, 32
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#  INSTRUCCION ADDI: TEST 25
#                 testnum, inst, val1, imm
#  TEST_IMM_ZERODEST( 25, addi, 33, 50 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 25

    #-- val1
    li  x1, 33

    #-- imm
    addi x0, x1, 50

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


#   
#   
#   TEST_IMM_OP( 10, slli, 0xffffffffffffc000, 0xffffffffffffffff, 14 );
#   TEST_IMM_OP( 11, slli, 0xffffffff80000000, 0xffffffffffffffff, 31 );

#   TEST_IMM_OP( 12, slli, 0x0000000021212121, 0x0000000021212121, 0  );
#   TEST_IMM_OP( 13, slli, 0x0000000042424242, 0x0000000021212121, 1  );
#   TEST_IMM_OP( 14, slli, 0x0000001090909080, 0x0000000021212121, 7  );
#   TEST_IMM_OP( 15, slli, 0x0000084848484000, 0x0000000021212121, 14 );
#   TEST_IMM_OP( 16, slli, 0x1090909080000000, 0x0000000021212121, 31 );

# #if __riscv_xlen == 64
#   TEST_IMM_OP( 50, slli, 0x8000000000000000, 0x0000000000000001, 63 );
#   TEST_IMM_OP( 51, slli, 0xffffff8000000000, 0xffffffffffffffff, 39 );
#   TEST_IMM_OP( 52, slli, 0x0909080000000000, 0x0000000021212121, 43 );
# #endif

#   #-------------------------------------------------------------
#   # Source/Destination tests
#   #-------------------------------------------------------------

#   TEST_IMM_SRC1_EQ_DEST( 17, slli, 0x00000080, 0x00000001, 7 );

#   #-------------------------------------------------------------
#   # Bypassing tests
#   #-------------------------------------------------------------

#   TEST_IMM_DEST_BYPASS( 18, 0, slli, 0x0000000000000080, 0x0000000000000001, 7  );
#   TEST_IMM_DEST_BYPASS( 19, 1, slli, 0x0000000000004000, 0x0000000000000001, 14 );
#   TEST_IMM_DEST_BYPASS( 20, 2, slli, 0x0000000080000000, 0x0000000000000001, 31 );

#   TEST_IMM_SRC1_BYPASS( 21, 0, slli, 0x0000000000000080, 0x0000000000000001, 7  );
#   TEST_IMM_SRC1_BYPASS( 22, 1, slli, 0x0000000000004000, 0x0000000000000001, 14 );
#   TEST_IMM_SRC1_BYPASS( 23, 2, slli, 0x0000000080000000, 0x0000000000000001, 31 );

#   TEST_IMM_ZEROSRC1( 24, slli, 0, 31 );
#   TEST_IMM_ZERODEST( 25, slli, 33, 20 );

