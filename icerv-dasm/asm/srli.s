#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SRLI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#            testnum,    val1,        imm
#   TEST_SRLI( 2,  0xffffffff80000000, 0  );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srli x14, x13, 0x00

    #-- x7: Valor del resultado esperado
    li x7, 0x80000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_SRLI( 3,  0xffffffff80000000, 1  );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srli x14, x13, 0x001

    #-- x7: Valor del resultado esperado
    li x7, 0x40000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_SRLI( 4,  0xffffffff80000000, 7  );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srli x14, x13, 0x007

    #-- x7: Valor del resultado esperado
    li x7, 0x01000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_SRLI( 5,  0xffffffff80000000, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srli x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0x20000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_SRLI( 6,  0xffffffff80000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x13, 0x80000001

    #-- Resultado obtenido
    srli x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x00000001

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_SRLI( 7,  0xffffffffffffffff, 0  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    srli x14, x13, 0

    #-- x7: Valor del resultado esperado
    li x7, 0xffffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#   TEST_SRLI( 8,  0xffffffffffffffff, 1  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    srli x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0x7fffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_SRLI( 9,  0xffffffffffffffff, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    srli x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0x01ffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_SRLI( 10, 0xffffffffffffffff, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    srli x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0x0003ffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,    imm
#   TEST_SRLI( 11, 0xffffffffffffffff, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    #-- Valor 1
    li x13, 0xffffffff

    #-- Resultado obtenido
    srli x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x00000001

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_SRLI( 12, 0x0000000021212121, 0  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    srli x14, x13, 0x0

    #-- x7: Valor del resultado esperado
    li x7, 0x21212121

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_SRLI( 13, 0x0000000021212121, 1  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 13

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    srli x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0x10909090

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#   TEST_SRLI( 14, 0x0000000021212121, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 14

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    srli x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0x00424242

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_SRLI( 15, 0x0000000021212121, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 15

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    srli x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0x00008484

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,            val1,               imm
#   TEST_SRLI( 16, 0x0000000021212121, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 16

    #-- Valor 1
    li x13, 0x21212121

    #-- Resultado obtenido
    srli x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#   TEST_IMM_SRC1_EQ_DEST( 17, srli, 0x01000000, 0x80000000, 7 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 17

    #-- Valor 1
    li x11, 0x80000000

    #-- Resultado obtenido
    srli x11, x11, 7

    #-- x7: Valor del resultado esperado
    li x7, 0x01000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,     val1,      imm 
#   TEST_IMM_DEST_BYPASS( 18, 0, srli, 0x01000000, 0x80000000, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 18

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srli x14, x1, 7

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x01000000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
#   TEST_IMM_DEST_BYPASS( 19, 1, srli, 0x00020000, 0x80000000, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 19

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srli x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00020000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,  val1, imm 
#   TEST_IMM_DEST_BYPASS( 20, 2, srli, 0x00000001, 0x80000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 20

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000001

      #-- Valor inmediato
      srli x14, x1, 31

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00000001
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,      val1, imm 
#   TEST_IMM_SRC1_BYPASS( 21, 0, srli, 0x01000000, 0x80000000, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 21

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srli x14, x1, 7

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x01000000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#   TEST_IMM_SRC1_BYPASS( 22, 1, srli, 0x00020000, 0x80000000, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 22

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srli x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00020000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#   TEST_IMM_SRC1_BYPASS( 23, 2, srli, 0x00000001, 0x80000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 23

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000001

      #-- Valor inmediato
      srli x14, x1, 31

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0x00000001
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, result, imm
#   TEST_IMM_ZEROSRC1( 24, srli, 0, 4 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 24

    #-- imm
    srli x1, x0, 4

    #-- Result
    li  x7, 0
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 25, srli, 33, 10 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 25

    #-- val1
    li  x1, 33

    #-- imm
    srli x0, x1, 10

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










