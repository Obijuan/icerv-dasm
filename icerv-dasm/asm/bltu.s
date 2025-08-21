#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE BLTU
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────


#───────────────────────────────────────────────────────────────────────
#               testnum, inst,         val1, val2
#   TEST_BR2_OP_TAKEN( 2, bltu, 0x00000000, 0x00000001 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- testnum
    li  x1, 0x00000000       #-- val1
    li  x2, 0x00000001       #-- val2
    bltu x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bltu x1, x2, 1b
    bne x0, x3, fail
3:

#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 3, bltu, 0xfffffffe, 0xffffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- testnum
    li  x1, 0xfffffffe       #-- val1
    li  x2, 0xffffffff       #-- val2
    bltu x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bltu x1, x2, 1b
    bne x0, x3, fail
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_TAKEN( 4, bltu, 0x00000000, 0xffffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- testnum
    li  x1, 0x00000000       #-- val1
    li  x2, 0xffffffff      #-- val2
    bltu x1, x2, 2f
    bne x0, x3, fail
1:  bne x0, x3, 3f
2:  bltu x1, x2, 1b
    bne x0, x3, fail
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst,          val1, val2
#   TEST_BR2_OP_NOTTAKEN( 5, bltu, 0x00000001, 0x00000000 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5
    li  x1, 0x00000001   #-- val1
    li  x2, 0x00000000   #-- val2
    bltu x1, x2, 1f
    bne x0, x3, 2f
1:  bne x0, x3, fail
2:  bltu x1, x2, 1b
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN( 6, bltu, 0xffffffff, 0xfffffffe );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6
    li  x1, 0xffffffff   #-- val1
    li  x2, 0xfffffffe   #-- val2
    bltu x1, x2, 1f
    bne x0, x3, 2f
1:  bne x0, x3, fail
2:  bltu x1, x2, 1b
3:


#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN( 7, bltu, 0xffffffff, 0x00000000 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7
    li  x1, 0xffffffff   #-- val1
    li  x2, 0x00000000   #-- val2
    bltu x1, x2, 1f
    bne x0, x3, 2f
1:  bne x0, x3, fail
2:  bltu x1, x2, 1b
3:

#───────────────────────────────────────────────────────────────────────
#               testnum, inst, val1, val2
#   TEST_BR2_OP_NOTTAKEN( 8, bltu, 0x80000000, 0x7fffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 8
    li  x1, 0x80000000   #-- val1
    li  x2, 0x7fffffff   #-- val2
    bltu x1, x2, 1f
    bne x0, x3, 2f
1:  bne x0, x3, fail
2:  bltu x1, x2, 1b
3:



#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 9,  0, 0, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 9
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
                 ## src1_nops
    li  x2, 0xefffffff    #-- val2
                ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 10, 0, 1, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 10
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
                 ## src1_nops
    li  x2, 0xefffffff  #-- val2
    nop         ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 11, 0, 2, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 11
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
                 ## src1_nops
    li  x2, 0xefffffff  #-- val2
    nop            ## src2_nops
    nop
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 12, 1, 0, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 12
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
    nop          ## src1_nops
    li  x2, 0xefffffff  #-- val2
                ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 13, 1, 1, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 13
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
    nop            ## src1_nops
    li  x2, 0xefffffff  #-- val2
    nop         ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 14, 2, 0, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 14
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
    nop            ## src1_nops
    nop
    li  x2, 0xefffffff  #-- val2
                ## src2_nops
     
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 15, 0, 0, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 15
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
              ## src1_nops
    li  x2, 0xefffffff  #-- val2
                ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 16, 0, 1, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 16
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
                 ## src1_nops
    li  x2, 0xefffffff  #-- val2
    nop            ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 17, 0, 2, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 17
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
                 ## src1_nops
     
    li  x2, 0xefffffff  #-- val2
    nop            ## src2_nops
    nop
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 18, 1, 0, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 18
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
    nop            ## src1_nops
     
    li  x2, 0xefffffff  #-- val2
                ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 19, 1, 1, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 19
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
    nop            ## src1_nops
     
    li  x2, 0xefffffff  #-- val2
    nop            ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#               testnum, src1_nops, src2_nops, inst, val1, val2
#   TEST_BR2_SRC12_BYPASS( 20, 2, 0, bltu, 0xf0000000, 0xefffffff );
#───────────────────────────────────────────────────────────────────────
    li  x3, 20
    li  x4, 0
1:  li  x1, 0xf0000000   #-- val1
    nop            ## src1_nops
    nop
    li  x2, 0xefffffff  #-- val2
                ## src2_nops
    bltu x1, x2, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b


#───────────────────────────────────────────────────────────────────────
#   TEST_CASE( 21, x1, 3, \
#     li  x1, 1; \
#     bltu x0, x1, 1f; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
# 1:  addi x1, x1, 1; \
#     addi x1, x1, 1; \
#───────────────────────────────────────────────────────────────────────
    li x3, 21

    li  x1, 1
    bltu x0, x1, 1f
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

