#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SLTU
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2,  sltu, 0, 0x00000000, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0x00000000  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3,  sltu, 0, 0x00000001, 0x00000001 );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 0x00000001  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4,  sltu, 1, 0x00000003, 0x00000007 );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x00000003  #-- val1
    li  x12, 0x00000007  #-- val2
    sltu x14, x11, x12

    li  x7, 1  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5,  sltu, 0, 0x00000007, 0x00000003 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0x00000007  #-- val1
    li  x12, 0x00000003  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 6,  sltu, 1, 0x00000000, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0xffff8000  #-- val2
    sltu x14, x11, x12

    li  x7, 1   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 7,  sltu, 0, 0x80000000, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00000000  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 8,  sltu, 1, 0x80000000, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0xffff8000  #-- val2
    sltu x14, x11, x12

    li  x7, 1   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 9,  sltu, 1, 0x00000000, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 9  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0x00007fff  #-- val2
    sltu x14, x11, x12

    li  x7, 1   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 10, sltu, 0, 0x7fffffff, 0x00000000 );
#────────────────────────────────────────────────
    li  x3, 10  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0x00000000  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 11, sltu, 0, 0x7fffffff, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 11  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0x00007fff  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 12, sltu, 0, 0x80000000, 0x00007fff );
#────────────────────────────────────────────────
    li  x3, 12  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00007fff  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 13, sltu, 1, 0x7fffffff, 0xffff8000 );
#────────────────────────────────────────────────
    li  x3, 13  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0xffff8000  #-- val2
    sltu x14, x11, x12

    li  x7, 1   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 14, sltu, 1, 0x00000000, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 14  #-- testnum

    li  x11, 0x00000000  #-- val1
    li  x12, 0xffffffff  #-- val2
    sltu x14, x11, x12

    li  x7, 1   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 15, sltu, 0, 0xffffffff, 0x00000001 );
#────────────────────────────────────────────────
    li  x3, 15  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0x00000001  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 16, sltu, 0, 0xffffffff, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 16  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0xffffffff  #-- val2
    sltu x14, x11, x12

    li  x7, 0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 17, sltu, 0, 14, 13 );
#────────────────────────────────────────────────
    li  x3, 17  #-- testnum

    li  x11, 14  #-- val1
    li  x12, 13  #-- val2
    sltu x11, x11, x12

    li  x7, 0   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 18, sltu, 1, 11, 13 );
#────────────────────────────────────────────────
    li  x3, 18  #-- testnum

    li  x11, 11  #-- val1
    li  x12, 13  #-- val2
    sltu x11, x11, x12

    li  x7, 1   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 19, sltu, 0, 13 );
#────────────────────────────────────────────────
    li  x3, 19  #-- testnum

    li  x11, 13 #-- val1 
    sltu x11, x11, x11; 

    li  x7, 0   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 20, 0, sltu, 1, 11, 13 );
#────────────────────────────────────────────────
    li  x3, 20 #-- testnum

    li  x4, 0
1:  li  x1, 11  #-- val1
    li  x2, 13  #-- val2
    sltu x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 1  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 21, 1, sltu, 0, 14, 13 );
#────────────────────────────────────────────────
    li  x3, 21 #-- testnum

    li  x4, 0
1:  li  x1, 14  #-- val1
    li  x2, 13  #-- val2
    sltu x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 22, 2, sltu, 1, 12, 13 );
#────────────────────────────────────────────────
    li  x3, 22 #-- testnum

    li  x4, 0
1:  li  x1, 1  #-- val1
    li  x2, 13  #-- val2
    sltu x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 1  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 23, 0, 0, sltu, 0, 14, 13 );
#────────────────────────────────────────────────
    li  x3, 23 #-- testnum

    li  x4, 0
1:  li  x1, 14 #-- val1
               ## src1_nops 
    li  x2, 13 #-- val2 
               ## src2_nops 
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 24, 0, 1, sltu, 1, 11, 13 );
#────────────────────────────────────────────────
    li  x3, 24 #-- testnum

    li  x4, 0
1:  li  x1, 11 #-- val1
               ## src1_nops 
    li  x2, 13 #-- val2 
    nop        ## src2_nops 
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 1  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 25, 0, 2, sltu, 0, 15, 13 );
#────────────────────────────────────────────────
    li  x3, 25 #-- testnum

    li  x4, 0
1:  li  x1, 15 #-- val1
               ## src1_nops 
    li  x2, 13 #-- val2 
    nop        ## src2_nops 
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 26, 1, 0, sltu, 1, 10, 13 );
#────────────────────────────────────────────────
    li  x3, 26 #-- testnum

    li  x4, 0
1:  li  x1, 10 #-- val1
    nop           ## src1_nops 
    li  x2, 13 #-- val2 
    nop           ## src2_nops 
    nop
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 1  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 27, 1, 1, sltu, 0, 16, 13 );
#────────────────────────────────────────────────
    li  x3, 27 #-- testnum

    li  x4, 0
1:  li  x1, 16 #-- val1
    nop        ## src1_nops 
    li  x2, 13 #-- val2 
    nop        ## src2_nops 
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 28, 2, 0, sltu, 1,  9, 13 );
#────────────────────────────────────────────────
    li  x3, 28 #-- testnum

    li  x4, 0
1:  li  x1, 9 #-- val1
    nop        ## src1_nops 
    nop
    li  x2, 13 #-- val2 
               ## src2_nops 
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 1  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 29, 0, 0, sltu, 0, 17, 13 );
#────────────────────────────────────────────────
    li  x3, 29 #-- testnum

    li  x4, 0
1:  li  x1, 17 #-- val1
               ## src1_nops 
    li  x2, 13 #-- val2 
               ## src2_nops 
    sltu x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 30, 0, 1, sltu, 1,  8, 13 );
#────────────────────────────────────────────────
    li  x3, 30  #-- testnum

    li  x4, 0
1:  li  x2, 13  #-- val2
                ## src1_nops 
     
    li x1, 8  #-- val1
    nop        ## src2_nops 
    sltu x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 1  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 31, 0, 2, sltu, 0, 18, 13 );
#────────────────────────────────────────────────
    li  x3, 31  #-- testnum

    li  x4, 0
1:  li  x2, 13  #-- val2
              ## src1_nops 
     
    li x1, 18  #-- val1
    nop           ## src2_nops 
    sltu x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 32, 1, 0, sltu, 1,  7, 13 );
#────────────────────────────────────────────────
    li  x3, 37  #-- testnum

    li  x4, 0
1:  li  x2, 13  #-- val2
    nop         ## src1_nops 
    li x1, 7  #-- val1
               ## src2_nops 
    sltu x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 1  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 33, 1, 1, sltu, 0, 19, 13 );
#────────────────────────────────────────────────
    li  x3, 33  #-- testnum

    li  x4, 0
1:  li  x2, 13  #-- val2
    nop         ## src1_nops 
    li x1, 19  #-- val1
               ## src2_nops 
    sltu x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 34, 2, 0, sltu, 1,  6, 13 );
#────────────────────────────────────────────────
    li  x3, 34  #-- testnum

    li  x4, 0
1:  li  x2, 13  #-- val2
    nop         ## src1_nops 
    li x1, 6  #-- val1
    nop          ## src2_nops 
    sltu x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 1  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 35, sltu, 1, -1 );
#────────────────────────────────────────────────
    li x3, 35  #-- testnum

    li x1, -1  #-- val
    sltu x2, x0, x1

    li  x7, 1  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 36, sltu, 0, -1 );
#────────────────────────────────────────────────
    li x3, 36  #-- testnum

    li x1, -1  #-- val
    sltu x2, x1, x0
    
    li  x7, 0  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 37, sltu, 0 );
#────────────────────────────────────────────────
    li x3, 37  #-- testnum

    sltu x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 38, sltu, 16, 30 );
#────────────────────────────────────────────────
    li x3, 38  #-- testnum

    li x1, 16  #-- val1
    li x2, 30  #-- val2
    sltu x0, x1, x2
    
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



#   #-------------------------------------------------------------
#   # Bypassing tests
#   #-------------------------------------------------------------









