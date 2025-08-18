#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SUB
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2,  sub, 0x00000000, 0x00000000, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0x00000000  #-- val2
    sub x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3,  sub, 0x00000000, 0x00000001, 0x00000001 );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 0x00000001  #-- val2
    sub x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4,  sub, 0xfffffffc, 0x00000003, 0x00000007 );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x00000003  #-- val1
    li  x12, 0x00000007  #-- val2
    sub x14, x11, x12

    li  x7, 0xfffffffc  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5,  sub, 0x00008000, 0x00000000, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0xffff8000  #-- val2
    sub x14, x11, x12

    li  x7, 0x00008000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 6,  sub, 0x80000000, 0x80000000, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00000000  #-- val2
    sub x14, x11, x12

    li  x7, 0x80000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 7,  sub, 0x80008000, 0x80000000, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0xffff8000  #-- val2
    sub x14, x11, x12

    li  x7, 0x80008000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 8,  sub, 0xffff8001, 0x00000000, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0x00007fff  #-- val2
    sub x14, x11, x12

    li  x7, 0xffff8001   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 9,  sub, 0x7fffffff, 0x7fffffff, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 9  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0x00000000  #-- val2
    sub x14, x11, x12

    li  x7, 0x7fffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 10, sub, 0x7fff8000, 0x7fffffff, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 10  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0x00007fff  #-- val2
    sub x14, x11, x12

    li  x7, 0x7fff8000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 11, sub, 0x7fff8001, 0x80000000, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 11  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00007fff  #-- val2
    sub x14, x11, x12

    li  x7, 0x7fff8001   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 12, sub, 0x80007fff, 0x7fffffff, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 12  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0xffff8000  #-- val2
    sub x14, x11, x12

    li  x7, 0x80007fff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 13, sub, 0x00000001, 0x00000000, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 13  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0xffffffff  #-- val2
    sub x14, x11, x12

    li  x7, 0x00000001   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 14, sub, 0xfffffffe, 0xffffffff, 0x00000001 );
#────────────────────────────────────────────────
    li  x3, 14  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0x00000001  #-- val2
    sub x14, x11, x12

    li  x7, 0xfffffffe   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 15, sub, 0x00000000, 0xffffffff, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 15  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0xffffffff  #-- val2
    sub x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 16, sub, 2, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 16  #-- testnum

    li  x11, 13  #-- val1
    li  x12, 11  #-- val2
    sub x11, x11, x12

    li  x7, 2   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 17, sub, 3, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 17  #-- testnum

    li  x11, 14  #-- val1
    li  x12, 11  #-- val2
    sub x11, x11, x12

    li  x7, 3   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 18, sub, 0, 13 );
#────────────────────────────────────────────────
    li  x3, 18  #-- testnum

    li  x11, 13 #-- val1 
    sub x11, x11, x11; 

    li  x7, 0   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 19, 0, sub, 2, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 19 #-- testnum

    li  x4, 0
1:  li  x1, 13  #-- val1
    li  x2, 11  #-- val2
    sub x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 2  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 20, 1, sub, 3, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 20 #-- testnum

    li  x4, 0
1:  li  x1, 14  #-- val1
    li  x2, 11  #-- val2
    sub x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 3  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 21, 2, sub, 4, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 21 #-- testnum

    li  x4, 0
1:  li  x1, 15  #-- val1
    li  x2, 11  #-- val2
    sub x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 4  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 22, 0, 0, sub, 2, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 22 #-- testnum

    li  x4, 0
1:  li  x1, 13 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
               ## src2_nops 
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 2  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 23, 0, 1, sub, 3, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 23 #-- testnum

    li  x4, 0
1:  li  x1, 14 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
    nop        ## src2_nops 
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 3  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 24, 0, 2, sub, 4, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 24 #-- testnum

    li  x4, 0
1:  li  x1, 15 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
    nop        ## src2_nops 
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 4  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 25, 1, 0, sub, 2, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 25 #-- testnum

    li  x4, 0
1:  li  x1, 13 #-- val1
    nop           ## src1_nops 
    li  x2, 11 #-- val2 
    nop           ## src2_nops 
    nop
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 2  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 26, 1, 1, sub, 3, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 26 #-- testnum

    li  x4, 0
1:  li  x1, 14 #-- val1
    nop        ## src1_nops 
    li  x2, 11 #-- val2 
    nop        ## src2_nops 
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 3  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 27, 2, 0, sub, 4, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 27 #-- testnum

    li  x4, 0
1:  li  x1, 15 #-- val1
    nop        ## src1_nops 
    nop
    li  x2, 11 #-- val2 
               ## src2_nops 
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 4  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 28, 0, 0, sub, 2, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 28 #-- testnum

    li  x4, 0
1:  li  x1, 13 #-- val1
               ## src1_nops 
    li  x2, 11 #-- val2 
               ## src2_nops 
    sub x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 2  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 29, 0, 1, sub, 3, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 29  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
                ## src1_nops 
     
    li x1, 14  #-- val1
    nop        ## src2_nops 
    sub x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 3  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 30, 0, 2, sub, 4, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 30  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
              ## src1_nops 
     
    li x1, 15  #-- val1
    nop           ## src2_nops 
    sub x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 4  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 31, 1, 0, sub, 2, 13, 11 );
#────────────────────────────────────────────────
    li  x3, 31  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    li x1, 13  #-- val1
               ## src2_nops 
    sub x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 2  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 32, 1, 1, sub, 3, 14, 11 );
#────────────────────────────────────────────────
    li  x3, 32  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    li x1, 14  #-- val1
               ## src2_nops 
    sub x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 3  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 33, 2, 0, sub, 4, 15, 11 );
#────────────────────────────────────────────────
    li  x3, 33  #-- testnum

    li  x4, 0
1:  li  x2, 11  #-- val2
    nop         ## src1_nops 
    li x1, 15  #-- val1
    nop          ## src2_nops 
    sub x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 4  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 34, sub, 15, -15 );
#────────────────────────────────────────────────
    li x3, 34  #-- testnum

    li x1, -15  #-- val
    sub x2, x0, x1

    li  x7, 15  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 35, sub, 32, 32 );
#────────────────────────────────────────────────
    li x3, 35  #-- testnum

    li x1, 32  #-- val
    sub x2, x1, x0
    
    li  x7, 32  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 36, sub, 0 );
#────────────────────────────────────────────────
    li x3, 36  #-- testnum

    sub x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 37, sub, 16, 30 );
#────────────────────────────────────────────────
    li x3, 37  #-- testnum

    li x1, 16  #-- val1
    li x2, 30  #-- val2
    sub x0, x1, x2
    
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


