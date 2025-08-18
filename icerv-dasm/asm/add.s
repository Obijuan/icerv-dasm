#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE ADD
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2,  add, 0x00000000, 0x00000000, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0x00000000  #-- val2
    add x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3,  add, 0x00000002, 0x00000001, 0x00000001 );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 0x00000001  #-- val2
    add x14, x11, x12

    li  x7, 0x00000002   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4,  add, 0x0000000a, 0x00000003, 0x00000007 );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x00000003  #-- val1
    li  x12, 0x00000007  #-- val2
    add x14, x11, x12

    li  x7, 0x00000000a  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5,  add, 0xffff8000, 0x00000000, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0xffff8000  #-- val2
    add x14, x11, x12

    li  x7, 0xffff8000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 6,  add, 0x80000000, 0x80000000, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x80000000  #-- val2
    add x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 7,  add, 0x7fff8000, 0x80000000, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0xffff8000  #-- val2
    add x14, x11, x12

    li  x7, 0x7fff8000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 8,  add, 0x00007fff, 0x00000000, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0x00007fff  #-- val2
    add x14, x11, x12

    li  x7, 0x00007fff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 9,  add, 0x7fffffff, 0x7fffffff, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 9  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0x00000000  #-- val2
    add x14, x11, x12

    li  x7, 0x7fffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 10, add, 0x80007ffe, 0x7fffffff, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 10  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0x00007fff  #-- val2
    add x14, x11, x12

    li  x7, 0x80007ffe   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 11, add, 0x80007fff, 0x80000000, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 11  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00007fff  #-- val2
    add x14, x11, x12

    li  x7, 0x80007fff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 12, add, 0x7fff7fff, 0x7fffffff, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 12  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0xffff8000  #-- val2
    add x14, x11, x12

    li  x7, 0x7fff7fff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 13, add, 0xffffffff, 0x00000000, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 13  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0xffffffff  #-- val2
    add x14, x11, x12

    li  x7, 0xffffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 14, add, 0x00000000, 0xffffffff, 0x00000001 );
#────────────────────────────────────────────────
    li  x3, 14  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0x00000001  #-- val2
    add x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#  TEST_RR_OP( 15, add, 0xfffffffe, 0xffffffff, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 15  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0xffffffff  #-- val2
    add x14, x11, x12

    li  x7, 0xfffffffe   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#  TEST_RR_OP( 16, add, 0x80000000, 0x00000001, 0x7fffffff );
#────────────────────────────────────────────────
    li  x3, 16  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 0x7fffffff  #-- val2
    add x14, x11, x12

    li  x7, 0x80000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_SRC1_EQ_DEST( 17, add, 24, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 17  #-- testnum

    li  x11, 13  #-- val1
    li  x12, 11  #-- val2
    add x11, x11, x12

    li  x7, 24   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_SRC2_EQ_DEST( 18, add, 25, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 18  #-- testnum

    li  x11, 14  #-- val1
    li  x12, 11  #-- val2
    add x12, x11, x12

    li  x7, 25   #--result
    bne x12, x7, fail


#────────────────────────────────────────────────
#                 testnum, inst,  result, val1, 
#   TEST_RR_SRC12_EQ_DEST( 19, add, 26, 13 );
#────────────────────────────────────────────────
    li  x3, 19  #-- testnum

    li  x11, 13  #-- val1
    add x11, x11, x11

    li  x7, 26   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 20, 0,     add,   24,     13, 11 );
#────────────────────────────────────────────────
    li  x3, 20 #-- testnum

    li  x4, 0
1:  li  x1, 13  #-- val1
    li  x2, 11  #-- val2
    add x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 24  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 21, 1, add, 25, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 21 #-- testnum

    li  x4, 0
1:  li  x1, 11  #-- val1
    li  x2, 14  #-- val2
    add x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 25  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 22, 2, add, 26, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 22 #-- testnum

    li  x4, 0
1:  li  x1, 11  #-- val1
    li  x2, 15  #-- val2
    add x14, x1, x2
    nop   ## nop_cycles
    nop
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 26  #-- result
    bne x6, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#  TEST_RR_SRC12_BYPASS( 23, 0, 0,      add,  24,      13, 11 );
#────────────────────────────────────────────────
    li  x3, 23 #-- testnum

    li  x4, 0
1:  li  x1, 13 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
               ## src2_nops 
    add x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 24  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#  TEST_RR_SRC12_BYPASS( 24, 0, 1, add, 25, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 24 #-- testnum

    li  x4, 0
1:  li  x1, 14 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
    nop        ## src2_nops 
    add x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 25  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#  TEST_RR_SRC12_BYPASS( 25, 0, 2, add, 26, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 25 #-- testnum

    li  x4, 0
1:  li  x1, 15 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
    nop           ## src2_nops 
    nop
    add x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 26  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#  TEST_RR_SRC12_BYPASS( 26, 1, 0, add, 24, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 26 #-- testnum

    li  x4, 0
1:  li  x1, 13 #-- val1
    nop        ## src1_nops 
    li  x2, 11 #-- val2 
               ## src2_nops 
    add x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 24  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#  TEST_RR_SRC12_BYPASS( 27, 1, 1, add, 25, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 27 #-- testnum

    li  x4, 0
1:  li  x1, 14 #-- val1
    nop        ## src1_nops 
    li  x2, 11 #-- val2 
    nop        ## src2_nops 
    add x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 25  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#  TEST_RR_SRC12_BYPASS( 28, 2, 0, add, 26, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 28 #-- testnum

    li  x4, 0
1:  li  x1, 15 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
               ## src2_nops 
    add x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 26  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 29, 0, 0, add, 24, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 29  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
                ## src1_nops 
     
    li x1, 13  #-- val1
               ## src2_nops 
    add x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 24  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 30, 0, 1, add, 25, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 30  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
              ## src1_nops 
     
    li x1, 14  #-- val1
    nop           ## src2_nops 
    add x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 25  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 31, 0, 2, add, 26, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 31  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    nop
    li x1, 15  #-- val1
               ## src2_nops 
    add x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 26  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 32, 1, 0, add, 24, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 32  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    li x1, 13  #-- val1
               ## src2_nops 
    add x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 24  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 33, 1, 1, add, 25, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 33  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    li x1, 14  #-- val1
    nop          ## src2_nops 
    add x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 25  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 34, 2, 0, add, 26, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 34  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    nop
    li x1, 15  #-- val1
               ## src2_nops 
    add x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 26  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 35, add, 15,    15 );
#────────────────────────────────────────────────
    li x3, 35  #-- testnum

    li x1, 15  #-- val
    add x2, x0, x1

    li  x7, 15  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 36, add, 32, 32 );
#────────────────────────────────────────────────
    li x3, 36  #-- testnum

    li x1, 32  #-- val
    add x2, x1, x0
    
    li  x7, 32  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 37, add, 0 );
#────────────────────────────────────────────────
    li x3, 37  #-- testnum

    add x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 38, add, 16, 30 );
#────────────────────────────────────────────────
    li x3, 38  #-- testnum

    li x1, 16  #-- val1
    li x2, 30  #-- val2
    add x0, x1, x2
    
    li  x7, 0  #-- Result
    bne x0, x7, fail


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


