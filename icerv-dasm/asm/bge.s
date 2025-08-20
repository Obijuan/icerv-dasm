#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE BGE
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 2, bge,  0,  0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- testnum
    li  x1, 0       #-- val1
    li  x2, 0       #-- val2
    bge x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bge x1, x2, 1b
    bne x0, x3, fail
3:

#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 3, bge,  1,  1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- testnum
    li  x1, 1       #-- val1
    li  x2, 1       #-- val2
    bge x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bge x1, x2, 1b
    bne x0, x3, fail
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 4, bge, -1, -1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- testnum
    li  x1, -1       #-- val1
    li  x2, -1      #-- val2
    bge x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bge x1, x2, 1b
    bne x0, x3, fail
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 5, bge,  1,  0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5  #-- testnum
    li  x1, 1       #-- val1
    li  x2, 0      #-- val2
    bge x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bge x1, x2, 1b
    bne x0, x3, fail
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 6, bge,  1, -1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6  #-- testnum
    li  x1, 1       #-- val1
    li  x2, -1      #-- val2
    bge x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bge x1, x2, 1b
    bne x0, x3, fail
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 7, bge, -1, -2 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7  #-- testnum
    li  x1, -1       #-- val1
    li  x2, -2      #-- val2
    bge x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bge x1, x2, 1b
    bne x0, x3, fail
3:

#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN(  8, bge,  0,  1 );
#───────────────────────────────────────────────────────────────────────
    li x3, 8
    li x1, 0   #-- val1
    li x2, 1   #-- val2
    bge x1, x2, 1f
    bne x0, x3, 2f
1:  bne x0, x3, fail
2:  bge x1, x2, 1b
3:

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN(  9, bge, -1,  1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 9
    li  x4, 0
1:  li  x1, -1   #-- val1
                ## src1_nops
    li  x2, 1  #-- val2
                ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN( 10, bge, -2, -1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 10
    li  x4, 0
1:  li  x1, -2   #-- val1
                ## src1_nops
    li  x2, -1  #-- val2
    nop         ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b


#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN( 11, bge, -2,  1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 11
    li  x4, 0
1:  li  x1, -2   #-- val1
                ## src1_nops
    li  x2, 1  #-- val2
    nop         ## src2_nops
    nop
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b


#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 12, 0, 0, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 12
    li  x4, 0
1:  li  x1, -1   #-- val1
                 ## src1_nops
    li  x2, 0    #-- val2
                ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 13, 0, 1, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 13
    li  x4, 0
1:  li  x1, -1   #-- val1
                 ## src1_nops
    li  x2, 0  #-- val2
    nop         ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 14, 0, 2, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 14
    li  x4, 0
1:  li  x1, -1   #-- val1
                 ## src1_nops
    li  x2, 0  #-- val2
    nop            ## src2_nops
    nop
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 15, 1, 0, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 15
    li  x4, 0
1:  li  x1, -1   #-- val1
    nop          ## src1_nops
    li  x2, 0  #-- val2
                ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 16, 1, 1, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 16
    li  x4, 0
1:  li  x1, -1   #-- val1
    nop            ## src1_nops
    li  x2, 0  #-- val2
    nop         ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 17, 2, 0, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 17
    li  x4, 0
1:  li  x1, -1   #-- val1
    nop            ## src1_nops
    nop
    li  x2, 0  #-- val2
                ## src2_nops
     
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 18, 0, 0, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 18
    li  x4, 0
1:  li  x1, -1   #-- val1
              ## src1_nops
    li  x2, 0  #-- val2
                ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 19, 0, 1, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 19
    li  x4, 0
1:  li  x1, -1   #-- val1
                 ## src1_nops
    li  x2, 0  #-- val2
    nop            ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 20, 0, 2, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 20
    li  x4, 0
1:  li  x1, -1   #-- val1
                 ## src1_nops
     
    li  x2, 0  #-- val2
    nop            ## src2_nops
    nop
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 21, 1, 0, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 20
    li  x4, 0
1:  li  x1, -1   #-- val1
    nop            ## src1_nops
     
    li  x2, 0  #-- val2
                ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 22, 1, 1, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 20
    li  x4, 0
1:  li  x1, -1   #-- val1
    nop            ## src1_nops
     
    li  x2, 0  #-- val2
    nop            ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 23, 2, 0, bge, -1, 0 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 20
    li  x4, 0
1:  li  x1, -1   #-- val1
    nop            ## src1_nops
    nop
    li  x2, 0  #-- val2
                ## src2_nops
    bge x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b


#───────────────────────────────────────────────────────────────────────
#   TEST_CASE( 24, x1, 3, \
#     li  x1, 1; \
#     bge x1, x0, 1f; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
# 1:  addi x1, x1, 1; \
#     addi x1, x1, 1; \
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 21

    li  x1, 1
    bge x1, x0, 1f
    addi x1, x1, 1
    addi x1, x1, 1
    addi x1, x1, 1
    addi x1, x1, 1
1:  addi x1, x1, 1
    addi x1, x1, 1

    li  x7, 3   #-- correctval
    bne x1, x7, fail

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


