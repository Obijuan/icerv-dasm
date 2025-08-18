#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SLL
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2,  sll, 0x00000001, 0x00000001, 0  );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 0x00000000  #-- val2
    sll x14, x11, x12

    li  x7, 0x00000001   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3,  sll, 0x00000002, 0x00000001, 1  );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 1  #-- val2
    sll x14, x11, x12

    li  x7, 0x00000002   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4,  sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 7  #-- val2
    sll x14, x11, x12

    li  x7, 0x00000080  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5,  sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 14  #-- val2
    sll x14, x11, x12

    li  x7, 0x00004000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 6,  sll, 0x80000000, 0x00000001, 31 );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00000000  #-- val2
    sll x14, x11, x12

    li  x7, 0x80000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 7,  sll, 0xffffffff, 0xffffffff, 0  );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0  #-- val2
    sll x14, x11, x12

    li  x7, 0xffffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 8,  sll, 0xfffffffe, 0xffffffff, 1  );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 1  #-- val2
    sll x14, x11, x12

    li  x7, 0xfffffffe   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 9,  sll, 0xffffff80, 0xffffffff, 7  );
#────────────────────────────────────────────────
    li  x3, 9  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 7  #-- val2
    sll x14, x11, x12

    li  x7, 0xffffff80   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 10, sll, 0xffffc000, 0xffffffff, 14 );
#────────────────────────────────────────────────
    li  x3, 10  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 14  #-- val2
    sll x14, x11, x12

    li  x7, 0xffffc000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 11, sll, 0x80000000, 0xffffffff, 31 );
#────────────────────────────────────────────────
    li  x3, 11  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 31  #-- val2
    sll x14, x11, x12

    li  x7, 0x80000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 12, sll, 0x21212121, 0x21212121, 0  );
#────────────────────────────────────────────────
    li  x3, 12  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0  #-- val2
    sll x14, x11, x12

    li  x7, 0x21212121   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 13, sll, 0x42424242, 0x21212121, 1  );
#────────────────────────────────────────────────
    li  x3, 13  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 1  #-- val2
    sll x14, x11, x12

    li  x7, 0x42424242   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 14, sll, 0x90909080, 0x21212121, 7  );
#────────────────────────────────────────────────
    li  x3, 14  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 7  #-- val2
    sll x14, x11, x12

    li  x7, 0x90909080   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 15, sll, 0x48484000, 0x21212121, 14 );
#────────────────────────────────────────────────
    li  x3, 15  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 14  #-- val2
    sll x14, x11, x12

    li  x7, 0x48484000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 16, sll, 0x80000000, 0x21212121, 31 );
#────────────────────────────────────────────────
    li  x3, 16  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 31  #-- val2
    sll x14, x11, x12

    li  x7, 0x80000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 17, sll, 0x21212121, 0x21212121, 0xffffffc0 );
#────────────────────────────────────────────────
    li  x3, 17  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffc0  #-- val2
    sll x14, x11, x12

    li  x7, 0x21212121   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 18, sll, 0x42424242, 0x21212121, 0xffffffc1 );
#────────────────────────────────────────────────
    li  x3, 18  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffc1  #-- val2
    sll x14, x11, x12

    li  x7, 0x42424242   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 19, sll, 0x90909080, 0x21212121, 0xffffffc7 );
#────────────────────────────────────────────────
    li  x3, 19  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffc7  #-- val2
    sll x14, x11, x12

    li  x7, 0x90909080   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 20, sll, 0x48484000, 0x21212121, 0xffffffce );
#────────────────────────────────────────────────
    li  x3, 20  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffce  #-- val2
    sll x14, x11, x12

    li  x7, 0x48484000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 22, sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 22  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 7  #-- val2
    sll x11, x11, x12

    li  x7, 0x00000080   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 23, sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 23  #-- testnum

    li  x11, 0x00000001  #-- val1
    li  x12, 14  #-- val2
    sll x11, x11, x12

    li  x7, 0x00004000   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 24, sll, 24, 3 );
#────────────────────────────────────────────────
    li  x3, 24  #-- testnum

    li  x11, 3 #-- val1 
    sll x11, x11, x11; 

    li  x7, 24   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 25, 0, sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 25 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001  #-- val1
    li  x2, 7  #-- val2
    sll x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x00000080  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 26, 1, sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 26 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001  #-- val1
    li  x2, 14  #-- val2
    sll x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x00004000  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 27, 2, sll, 0x80000000, 0x00000001, 31 );
#────────────────────────────────────────────────
    li  x3, 27 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001  #-- val1
    li  x2, 31  #-- val2
    sll x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x80000000  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 28, 0, 0, sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 28 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
               ## src1_nops 
    li  x2, 7 #-- val2 
               ## src2_nops 
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000080  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 29, 0, 1, sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 29 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
               ## src1_nops 
    li  x2, 14 #-- val2 
    nop        ## src2_nops 
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00004000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 30, 0, 2, sll, 0x80000000, 0x00000001, 31 );
#────────────────────────────────────────────────
    li  x3, 30 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
               ## src1_nops 
    li  x2, 31 #-- val2 
    nop        ## src2_nops 
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x80000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 31, 1, 0, sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 31 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
    nop           ## src1_nops 
    li  x2, 7 #-- val2 
    nop           ## src2_nops 
    nop
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000080  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 32, 1, 1, sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 32 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
    nop        ## src1_nops 
    li  x2, 14 #-- val2 
    nop        ## src2_nops 
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00004000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 33, 2, 0, sll, 0x80000000, 0x00000001, 31 );
#────────────────────────────────────────────────
    li  x3, 33 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
    nop        ## src1_nops 
    nop
    li  x2, 31 #-- val2 
               ## src2_nops 
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x80000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 34, 0, 0, sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 34 #-- testnum

    li  x4, 0
1:  li  x1, 0x00000001 #-- val1
               ## src1_nops 
    li  x2, 7 #-- val2 
               ## src2_nops 
    sll x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000080  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 35, 0, 1, sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 35  #-- testnum

    li  x4, 0
1:  li  x2, 14  #-- val2
                ## src1_nops 
     
    li x1, 0x00000001  #-- val1
    nop        ## src2_nops 
    sll x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00004000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 36, 0, 2, sll, 0x80000000, 0x00000001, 31 );
#────────────────────────────────────────────────
    li  x3, 36  #-- testnum

    li  x4, 0
1:  li  x2, 31  #-- val2
              ## src1_nops 
     
    li x1, 0x00000001  #-- val1
    nop           ## src2_nops 
    sll x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x80000000  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 37, 1, 0, sll, 0x00000080, 0x00000001, 7  );
#────────────────────────────────────────────────
    li  x3, 37  #-- testnum

    li  x4, 0
1:  li  x2, 7  #-- val2
    nop         ## src1_nops 
    li x1, 0x00000001  #-- val1
               ## src2_nops 
    sll x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000080  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 38, 1, 1, sll, 0x00004000, 0x00000001, 14 );
#────────────────────────────────────────────────
    li  x3, 38  #-- testnum

    li  x4, 0
1:  li  x2, 14  #-- val2
    nop         ## src1_nops 
    li x1, 0x00000001  #-- val1
               ## src2_nops 
    sll x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00004000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 39, 2, 0, sll, 0x80000000, 0x00000001, 31 );
#────────────────────────────────────────────────
    li  x3, 39  #-- testnum

    li  x4, 0
1:  li  x2, 31  #-- val2
    nop         ## src1_nops 
    li x1, 0x00000001  #-- val1
    nop          ## src2_nops 
    sll x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x80000000  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 40, sll, 0, 15 );
#────────────────────────────────────────────────
    li x3, 40  #-- testnum

    li x1, 15  #-- val
    sll x2, x0, x1

    li  x7, 0  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 41, sll, 32, 32 );
#────────────────────────────────────────────────
    li x3, 41  #-- testnum

    li x1, 32  #-- val
    sll x2, x1, x0
    
    li  x7, 32  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 42, sll, 0 );
#────────────────────────────────────────────────
    li x3, 42  #-- testnum

    sll x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 43, sll, 1024, 2048 );
#────────────────────────────────────────────────
    li x3, 43  #-- testnum

    li x1, 1024  #-- val1
    li x2, 2048  #-- val2
    sll x0, x1, x2
    
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





