#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SLTI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum,    inst,  result, val1,   imm
# TEST_IMM_OP( 2,  sltiu, 0, 0x0000000000000000, 0x000 );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido
    sltiu x14, x13, 0x00

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#  TEST_IMM_OP( 3,  sltiu, 0, 0x0000000000000001, 0x001 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x00000001

    #-- Resultado obtenido
    sltiu x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 4,  sltiu, 1, 0x0000000000000003, 0x007 );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x00000003

    #-- Resultado obtenido
    sltiu x14, x13, 0x007

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#  TEST_IMM_OP( 5,  sltiu, 0, 0x0000000000000007, 0x003 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0x00000007

    #-- Resultado obtenido
    sltiu x14, x13, 3

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_IMM_OP( 6,  sltiu, 1, 0x0000000000000000, 0x800 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido
    sltiu x14, x13, 0xfffff800

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_IMM_OP( 7,  sltiu, 0, 0xffffffff80000000, 0x000 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    sltiu x14, x13, 0x000

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
# TEST_IMM_OP( 8,  sltiu, 1, 0xffffffff80000000, 0x800 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    sltiu x14, x13, 0xfffff800

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 9,  sltiu, 1, 0x0000000000000000, 0x7ff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido
    sltiu x14, x13, 0x7ff

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 10, sltiu, 0, 0x000000007fffffff, 0x000 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    sltiu x14, x13, 0x000

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,    imm
#  TEST_IMM_OP( 11, sltiu, 0, 0x000000007fffffff, 0x7ff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    sltiu x14, x13, 0x7ff

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_IMM_OP( 12, sltiu, 0, 0xffffffff80000000, 0x7ff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    sltiu x14, x13, 0x7ff

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#  TEST_IMM_OP( 13, sltiu, 1, 0x000000007fffffff, 0x800 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 13

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    sltiu x14, x13, 0xfffff800

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#  TEST_IMM_OP( 14, sltiu, 1, 0x0000000000000000, 0xfff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 14

    #-- Valor 1
    li x13, 0x00000000

    #-- Resultado obtenido
    sltiu x14, x13, 0xffffffff

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#  TEST_IMM_OP( 15, sltiu, 0, 0xffffffffffffffff, 0x001 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 15

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    sltiu x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,            val1,               imm
#  TEST_IMM_OP( 16, sltiu, 0, 0xffffffffffffffff, 0xfff );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 16

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    sltiu x14, x13, 0xffffffff

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#  TEST_IMM_SRC1_EQ_DEST( 17, sltiu, 1, 11, 13 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 17

    #-- Valor 1
    li x11, 11

    #-- Resultado obtenido
    sltiu x11, x11, 13

    #-- x7: Valor del resultado esperado
    li x7, 1

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,     val1,      imm 
#   TEST_IMM_DEST_BYPASS( 18, 0, sltiu, 0, 15, 10 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 18

    li x4, 0

    #-- Valor 1
1:  li x1, 15

      #-- Valor inmediato
      sltiu x14, x1, 10

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
# TEST_IMM_DEST_BYPASS( 19, 1, sltiu, 1, 10, 16 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 19

    li x4, 0

    #-- Valor 1
1:  li x1, 10

      #-- Valor inmediato
      sltiu x14, x1, 16

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 1
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,  val1, imm 
#  TEST_IMM_DEST_BYPASS( 20, 2, sltiu, 0, 16,  9 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 20

    li x4, 0

    #-- Valor 1
1:  li x1, 16

      #-- Valor inmediato
      sltiu x14, x1, 9

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1, imm 
# TEST_IMM_SRC1_BYPASS( 21, 0, sltiu, 1, 11, 15 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 21

    li x4, 0

    #-- Valor 1
1:  li x1, 11

      #-- Valor inmediato
      sltiu x14, x1, 15

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 1
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
# TEST_IMM_SRC1_BYPASS( 22, 1, sltiu, 0, 17,  8 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 22

    li x4, 0

    #-- Valor 1
1:  li x1, 17

      #-- Valor inmediato
      sltiu x14, x1, 8

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#  TEST_IMM_SRC1_BYPASS( 23, 2, sltiu, 1, 12, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 23

    li x4, 0

    #-- Valor 1
1:  li x1, 12

      #-- Valor inmediato
      sltiu x14, x1, 14

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
#  TEST_IMM_ZEROSRC1( 24, sltiu, 1, 0xfff );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 24

    #-- imm
    sltiu x1, x0, 0xffffffff

    #-- Result
    li  x7, 1
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 25, sltiu, 0x00ff00ff, 0xfff );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 25

    #-- val1
    li  x1, 0x00ff00ff

    #-- imm
    sltiu x0, x1, 0xffffffff

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


