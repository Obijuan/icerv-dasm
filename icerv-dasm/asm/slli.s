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

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 10, slli, 0xffffc000, 0xffffffff, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    slli x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0xffffc000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,    imm
#  TEST_IMM_OP( 11, slli, 0x80000000, 0xffffffff, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    slli x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x80000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 12, slli, 0x21212121, 0x21212121, 0  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    slli x14, x13, 0x0

    #-- x7: Valor del resultado esperado
    li x7, 0x21212121

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 13, slli, 0x42424242, 0x21212121, 1  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 13

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    slli x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0x42424242

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#  TEST_IMM_OP( 14, slli, 0x90909080, 0x21212121, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 14

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    slli x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0x90909080

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#  TEST_IMM_OP( 15, slli, 0x48484000, 0x21212121, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 15

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    slli x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0x48484000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 16, slli, 0x80000000, 0x21212121, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 16

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    slli x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x80000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#  TEST_IMM_SRC1_EQ_DEST( 17, slli, 0x00000080, 0x00000001, 7 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 17

    #-- Valor 1
    li x11, 0x00000001

    #-- Resultado obtenido
    slli x11, x11, 7

    #-- x7: Valor del resultado esperado
    li x7, 0x00000080

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,     val1,      imm 
#  TEST_IMM_DEST_BYPASS( 18, 0, slli, 0x00000080, 0x00000001, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 18

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00000001

      #-- Valor inmediato
      slli x14, x1, 7

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00000080
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
#  TEST_IMM_DEST_BYPASS( 19, 1, slli, 0x00004000, 0x00000001, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 19

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00000001

      #-- Valor inmediato
      slli x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00004000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,  val1, imm 
#  TEST_IMM_DEST_BYPASS( 20, 2, slli, 0x0000000080000000, 0x00000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 20

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00000001

      #-- Valor inmediato
      slli x14, x1, 31

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x80000000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1, imm 
#   TEST_IMM_SRC1_BYPASS( 21, 0, slli, 0x0000000000000080, 0x00000001, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 21

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00000001

      #-- Valor inmediato
      slli x14, x1, 7

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00000080
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 22, 1, slli, 0x0000000000004000, 0x00000001, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 22

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00000001

      #-- Valor inmediato
      slli x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00004000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 23, 2, slli, 0x0000000080000000, 0x00000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 23

    li x4, 0

    #-- Valor 1
1:  li x1, 0x00000001

      #-- Valor inmediato
      slli x14, x1, 31

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x80000000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, result, imm
#  TEST_IMM_ZEROSRC1( 24, slli, 0, 31 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 24

    #-- imm
    slli x1, x0, 31

    #-- Result
    li  x7, 0
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 25, slli, 33, 20 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 25

    #-- val1
    li  x1, 33

    #-- imm
    slli x0, x1, 20

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


