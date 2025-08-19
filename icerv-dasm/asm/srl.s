#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SRL
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 2,  0x80000000, 0  );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0  #-- val2
    srl x14, x11, x12

    li  x7,  0x80000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 3,  0x80000000, 1  );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 1  #-- val2
    srl x14, x11, x12

    li  x7,  0x40000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 4,  0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 7  #-- val2
    srl x14, x11, x12

    li  x7, 0x01000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 5,  0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 14  #-- val2
    srl x14, x11, x12

    li  x7, 0x00020000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 6,  0x80000001, 31 );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0x80000001  #-- val1
    li  x12, 31  #-- val2
    srl x14, x11, x12

    li  x7, 0x00000001   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 7,  0xffffffff, 0  );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0  #-- val2
    srl x14, x11, x12

    li  x7, 0xffffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 8,  0xffffffff, 1  );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 1  #-- val2
    srl x14, x11, x12

    li  x7, 0x7fffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 9,  0xffffffff, 7  );
#────────────────────────────────────────────────
    li  x3, 9  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 7  #-- val2
    srl x14, x11, x12

    li  x7, 0x1ffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 10, 0xffffffff, 14 );
#────────────────────────────────────────────────
    li  x3, 10  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 14  #-- val2
    srl x14, x11, x12

    li  x7, 0x0003ffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 11, 0xffffffff, 31 );
#────────────────────────────────────────────────
    li  x3, 11  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 31  #-- val2
    srl x14, x11, x12

    li  x7, 0x00000001   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 12, 0x21212121, 0  );
#────────────────────────────────────────────────
    li  x3, 12  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0  #-- val2
    srl x14, x11, x12

    li  x7, 0x21212121   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 13, 0x21212121, 1  );
#────────────────────────────────────────────────
    li  x3, 13  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 1  #-- val2
    srl x14, x11, x12

    li  x7, 0x10909090   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 14, 0x21212121, 7  );
#────────────────────────────────────────────────
    li  x3, 14  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 7  #-- val2
    srl x14, x11, x12

    li  x7, 0x00424242   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 15, 0x21212121, 14 );
#────────────────────────────────────────────────
    li  x3, 15  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 14  #-- val2
    srl x14, x11, x12

    li  x7, 0x00008484   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum,    val1,   val2
#   TEST_SRL( 16, 0x21212121, 31 );
#────────────────────────────────────────────────
    li  x3, 16  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 31  #-- val2
    srl x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail



#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 17, srl, 0x021212121, 0x021212121, 0xfffffffc0 );
#────────────────────────────────────────────────
    li  x3, 17  #-- testnum

    li  x11, 0x021212121  #-- val1
    li  x12, 0xfffffffc0  #-- val2
    srl x14, x11, x12

    li  x7, 0x021212121   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 18, srl, 0x10909090, 0x21212121, 0xffffffc1 );
#────────────────────────────────────────────────
    li  x3, 18  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffc1  #-- val2
    srl x14, x11, x12

    li  x7, 0x10909090  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 19, srl, 0x00424242, 0x21212121, 0xffffffc7 );
#────────────────────────────────────────────────
    li  x3, 19  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffc7  #-- val2
    srl x14, x11, x12

    li  x7, 0x00424242   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 20, srl, 0x00008484, 0x21212121, 0xffffffce );
#────────────────────────────────────────────────
    li  x3, 29  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffce  #-- val2
    srl x14, x11, x12

    li  x7, 0x00008484   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 21, srl, 0x00000000, 0x21212121, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 21  #-- testnum

    li  x11, 0x21212121  #-- val1
    li  x12, 0xffffffff  #-- val2
    srl x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 22, srl, 0x01000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 22  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 7  #-- val2
    srl x11, x11, x12

    li  x7, 0x01000000   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 23, srl, 0x00020000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 23  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 14  #-- val2
    srl x11, x11, x12

    li  x7, 0x00020000   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 24, srl, 0, 7 );
#────────────────────────────────────────────────
    li  x3, 24  #-- testnum

    li  x11, 7 #-- val1 
    srl x11, x11, x11; 

    li  x7, 0   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 25, 0, srl, 0x01000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 25 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000  #-- val1
    li  x2, 7  #-- val2
    srl x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x01000000  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 26, 1, srl, 0x00020000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 26 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000  #-- val1
    li  x2, 14  #-- val2
    srl x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x00020000  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 27, 2, srl, 0x00000001, 0x80000000, 31 );
#────────────────────────────────────────────────
    li  x3, 27 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000  #-- val1
    li  x2, 31  #-- val2
    srl x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x00000001  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 28, 0, 0, srl, 0x01000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 28 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
               ## src1_nops 
    li  x2, 7 #-- val2 
               ## src2_nops 
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x01000000  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 29, 0, 1, srl, 0x00020000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 29 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
               ## src1_nops 
    li  x2, 14 #-- val2 
    nop        ## src2_nops 
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00020000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 30, 0, 2, srl, 0x00000001, 0x80000000, 31 );
#────────────────────────────────────────────────
    li  x3, 30 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
               ## src1_nops 
    li  x2, 31 #-- val2 
    nop        ## src2_nops 
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000001  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 31, 1, 0, srl, 0x01000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 31 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
    nop           ## src1_nops 
    li  x2, 7 #-- val2 
    nop           ## src2_nops 
    nop
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x01000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 32, 1, 1, srl, 0x00020000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 32 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
    nop        ## src1_nops 
    li  x2, 14 #-- val2 
    nop        ## src2_nops 
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00020000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 33, 2, 0, srl, 0x00000001, 0x80000000, 31 );
#────────────────────────────────────────────────
    li  x3, 33 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
    nop        ## src1_nops 
    nop
    li  x2, 31 #-- val2 
               ## src2_nops 
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000001  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 34, 0, 0, srl, 0x01000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 34 #-- testnum

    li  x4, 0
1:  li  x1, 0x80000000 #-- val1
               ## src1_nops 
    li  x2, 7 #-- val2 
               ## src2_nops 
    srl x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x01000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 35, 0, 1, srl, 0x00020000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 35  #-- testnum

    li  x4, 0
1:  li  x2, 14  #-- val2
                ## src1_nops 
     
    li x1, 0x80000000  #-- val1
    nop        ## src2_nops 
    srl x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00020000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 36, 0, 2, srl, 0x00000001, 0x80000000, 31 );
#────────────────────────────────────────────────
    li  x3, 36  #-- testnum

    li  x4, 0
1:  li  x2, 31  #-- val2
              ## src1_nops 
     
    li x1, 0x80000000  #-- val1
    nop           ## src2_nops 
    srl x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000001  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 37, 1, 0, srl, 0x01000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 37  #-- testnum

    li  x4, 0
1:  li  x2, 7  #-- val2
    nop         ## src1_nops 
    li x1, 0x80000000  #-- val1
               ## src2_nops 
    srl x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x01000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 38, 1, 1, srl, 0x00020000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 38  #-- testnum

    li  x4, 0
1:  li  x2, 14  #-- val2
    nop         ## src1_nops 
    li x1, 0x80000000  #-- val1
               ## src2_nops 
    srl x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00020000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 39, 2, 0, srl, 0x00000001, 0x80000000, 31 );
#────────────────────────────────────────────────
    li  x3, 39  #-- testnum

    li  x4, 0
1:  li  x2, 31  #-- val2
    nop         ## src1_nops 
    li x1, 0x80000000  #-- val1
    nop          ## src2_nops 
    srl x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00000001  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 40, srl, 0, 15 );
#────────────────────────────────────────────────
    li x3, 40  #-- testnum

    li x1, 15  #-- val
    srl x2, x0, x1

    li  x7, 0  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 41, srl, 32, 32 );
#────────────────────────────────────────────────
    li x3, 41  #-- testnum

    li x1, 32  #-- val
    srl x2, x1, x0
    
    li  x7, 32  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 42, srl, 0 );
#────────────────────────────────────────────────
    li x3, 42  #-- testnum

    srl x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 43, srl, 1024, 2048 );
#────────────────────────────────────────────────
    li x3, 43  #-- testnum

    li x1, 1024  #-- val1
    li x2, 2048  #-- val2
    srl x0, x1, x2
    
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


