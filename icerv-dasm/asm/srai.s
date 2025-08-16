#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SRAI
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#            testnum,  inst,  result,  val1,    imm
#   TEST_IMM_OP( 2,  srai, 00000000, 0x00000000, 0  );
#───────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 2

    #-- Valor 1
    li x13, 0

    #-- Resultado obtenido
    srai x14, x13, 0

    #-- x7: Valor del resultado esperado
    li x7, 0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_IMM_OP( 3,  srai, 0xc0000000, 0x80000000, 1  );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 3

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srai x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0xc0000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,     imm
#   TEST_IMM_OP( 4,  srai, 0xff000000, 0x80000000, 7  );
#────────────────────────────────────────────────

    #-- Numero de test
    li x3, 4

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srai x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0xff000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_IMM_OP( 5,  srai, 0xfffe0000, 0x80000000, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 5

    #-- Valor 1
    li x13, 0x80000000

    #-- Resultado obtenido
    srai x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0xfffe0000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_IMM_OP( 6,  srai, 0xffffffffffffffff, 0xffffffff80000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 6

    #-- Valor 1
    li x13, 0x80000001

    #-- Resultado obtenido
    srai x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0xffffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,      imm
#   TEST_IMM_OP( 7,  srai, 0x7fffffff, 0x7fffffff, 0  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 7

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    srai x14, x13, 0

    #-- x7: Valor del resultado esperado
    li x7, 0x7fffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#   TEST_IMM_OP( 8,  srai, 0x3fffffff, 0x7fffffff, 1  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 8

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    srai x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0x3fffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_IMM_OP( 9,  srai, 0x00ffffff, 0x7fffffff, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 9

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    srai x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0x00ffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_IMM_OP( 10, srai, 0x0001ffff, 0x7fffffff, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 10

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    srai x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0x0001ffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,    imm
#   TEST_IMM_OP( 11, srai, 0x00000000, 0x7fffffff, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 11

    #-- Valor 1
    li x13, 0x7fffffff

    #-- Resultado obtenido
    srai x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0x00000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_IMM_OP( 12, srai, 0x81818181, 0x81818181, 0  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 12

    #-- Valor 1
    li x13, 0x81818181

    #-- Resultado obtenido
    srai x14, x13, 0

    #-- x7: Valor del resultado esperado
    li x7, 0x81818181

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,    imm
#   TEST_IMM_OP( 13, srai, 0xc0c0c0c0, 0x81818181, 1  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 13

    #-- Valor 1
    li x13, 0x81818181

    #-- Resultado obtenido
    srai x14, x13, 1

    #-- x7: Valor del resultado esperado
    li x7, 0xc0c0c0c0

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,   imm
#   TEST_IMM_OP( 14, srai, 0xff030303, 0x81818181, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 14

    #-- Valor 1
    li x13, 0x81818181

    #-- Resultado obtenido
    srai x14, x13, 7

    #-- x7: Valor del resultado esperado
    li x7, 0xff030303

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,      val1,     imm
#   TEST_IMM_OP( 15, srai, 0xfffe0606, 0x81818181, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 15

    #-- Valor 1
    li x13, 0x81818181

    #-- Resultado obtenido
    srai x14, x13, 14

    #-- x7: Valor del resultado esperado
    li x7, 0xfffe0606

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#         testnum,  inst,  result,       val1,    imm
#   TEST_IMM_OP( 16, srai, 0xffffffff, 0x81818181, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 16

    #-- Valor 1
    li x13, 0x81818181

    #-- Resultado obtenido
    srai x14, x13, 31

    #-- x7: Valor del resultado esperado
    li x7, 0xffffffff

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                  testnum,  inst,  result,      val1,    imm
#   TEST_IMM_SRC1_EQ_DEST( 17, srai, 0xff000000, 0x80000000, 7 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 17

    #-- Valor 1
    li x11, 0x80000000

    #-- Resultado obtenido
    srai x11, x11, 7

    #-- x7: Valor del resultado esperado
    li x7, 0xff000000

    #-- Comprobar el resultado obtenido (x14) con el esperado (x7)
    bne x11, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,     val1,      imm 
#   TEST_IMM_DEST_BYPASS( 18, 0, srai, 0xff000000, 0x80000000, 7  );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 18

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srai x14, x1, 7

      #TEST_INSERT_NOPS_ nop_cycles 
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xff000000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result,       val1,    imm 
#   TEST_IMM_DEST_BYPASS( 19, 1, srai, 0xfffe0000, 0x80000000, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 19

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srai x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xfffe0000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst,    result,     val1,    imm 
#   TEST_IMM_DEST_BYPASS( 20, 2, srai, 0xffffffff, 0x80000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 20

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000001

      #-- Valor inmediato
      srai x14, x1, 31

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xffffffff
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst,    result,      val1,   imm 
#   TEST_IMM_SRC1_BYPASS( 21, 0, srai, 0xff000000, 0x80000000, 7 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 21

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srai x14, x1, 7

      #TEST_INSERT_NOPS_ nop_cycles 
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xff000000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#   TEST_IMM_SRC1_BYPASS( 22, 1, srai, 0xfffffffffffe0000, 0xffffffff80000000, 14 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 22

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000000

      #-- Valor inmediato
      srai x14, x1, 14

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xfffe0000
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#          testnum, nop_cycles, inst, result, val1, imm 
#   TEST_IMM_SRC1_BYPASS( 23, 2, srai, 0xffffffffffffffff, 0xffffffff80000001, 31 );
#───────────────────────────────────────────────────────────────────────────

    #-- Numero de test
    li x3, 23

    li x4, 0

    #-- Valor 1
1:  li x1, 0x80000001

      #-- Valor inmediato
      srai x14, x1, 31

      #TEST_INSERT_NOPS_ nop_cycles 
      nop
      nop
      
      addi  x6, x14, 0
      addi  x4, x4, 1
      li  x5, 2
      bne x4, x5, 1b

    #-- Resultado esperado 
    li  x7, 0xffffffff
    bne x6, x7, fail

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, result, imm
#   TEST_IMM_ZEROSRC1( 24, srai, 0, 4 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 24

    #-- imm
    srai x1, x0, 4

    #-- Result
    li  x7, 0
    bne x1, x7, fail;

#───────────────────────────────────────────────────────────────────────────
#                 testnum, inst, val1, imm
#   TEST_IMM_ZERODEST( 25, srai, 33, 10 );
#───────────────────────────────────────────────────────────────────────────
    #-- Testnum
    li  x3, 25

    #-- val1
    li  x1, 33

    #-- imm
    srai x0, x1, 10

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











