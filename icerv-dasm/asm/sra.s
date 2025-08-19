#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SRA
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2,  sra, 0x80000000, 0x80000000, 0  );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 0x00000000  #-- val2
    srl x14, x11, x12

    li  x7, 0x80000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3,  sra, 0xc0000000, 0x80000000, 1  );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 1  #-- val2
    srl x14, x11, x12

    li  x7, 0xc0000000   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4,  sra, 0xff000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 7  #-- val2
    srl x14, x11, x12

    li  x7, 0xff000000  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5,  sra, 0xfffe0000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 14  #-- val2
    srl x14, x11, x12

    li  x7, 0xfffe0000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 6,  sra, 0xffffffff, 0x80000001, 31 );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0x80000001  #-- val1
    li  x12, 31  #-- val2
    srl x14, x11, x12

    li  x7, 0xffffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 7,  sra, 0x7fffffff, 0x7fffffff, 0  );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 0  #-- val2
    srl x14, x11, x12

    li  x7, 0x7fffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 8,  sra, 0x3fffffff, 0x7fffffff, 1  );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 1  #-- val2
    srl x14, x11, x12

    li  x7, 0x3fffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 9,  sra, 0x00ffffff, 0x7fffffff, 7  );
#────────────────────────────────────────────────
    li  x3, 9  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 7  #-- val2
    srl x14, x11, x12

    li  x7, 0x00ffffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 10, sra, 0x0001ffff, 0x7fffffff, 14 );
#────────────────────────────────────────────────
    li  x3, 10  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 14  #-- val2
    srl x14, x11, x12

    li  x7, 0x0001ffff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 11, sra, 0x00000000, 0x7fffffff, 31 );
#────────────────────────────────────────────────
    li  x3, 11  #-- testnum

    li  x11, 0x7fffffff  #-- val1
    li  x12, 31  #-- val2
    srl x14, x11, x12

    li  x7, 0x00000000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 12, sra, 0x81818181, 0x81818181, 0  );
#────────────────────────────────────────────────
    li  x3, 12  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 0  #-- val2
    srl x14, x11, x12

    li  x7, 0x81818181   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 13, sra, 0xc0c0c0c0, 0x81818181, 1  );
#────────────────────────────────────────────────
    li  x3, 13  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 1  #-- val2
    srl x14, x11, x12

    li  x7, 0xc0c0c0c0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 14, sra, 0xff030303, 0x81818181, 7  );
#────────────────────────────────────────────────
    li  x3, 14  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 7  #-- val2
    srl x14, x11, x12

    li  x7, 0xff030303   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 15, sra, 0xfffe0606, 0x81818181, 14 );
#────────────────────────────────────────────────
    li  x3, 15  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 14  #-- val2
    srl x14, x11, x12

    li  x7, 0xfffe0606   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 16, sra, 0xffffffff, 0x81818181, 31 );
#────────────────────────────────────────────────
    li  x3, 16  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 31  #-- val2
    srl x14, x11, x12

    li  x7, 0xffffffff   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 17, sra, 0x81818181, 0x81818181, 0xffffffc0 );
#────────────────────────────────────────────────
    li  x3, 17  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 0xffffffc0  #-- val2
    srl x14, x11, x12

    li  x7, 0x81818181   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 18, sra, 0xc0c0c0c0, 0x81818181, 0xffffffc1 );
#────────────────────────────────────────────────
    li  x3, 18  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 0xffffffc1  #-- val2
    srl x14, x11, x12

    li  x7, 0xc0c0c0c0   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 19, sra, 0xff030303, 0x81818181, 0xffffffc7 );
#────────────────────────────────────────────────
    li  x3, 19  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 0xffffffc7  #-- val2
    srl x14, x11, x12

    li  x7, 0xff030303   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 20, sra, 0xfffe0606, 0x81818181, 0xffffffce );
#────────────────────────────────────────────────
    li  x3, 20  #-- testnum

    li  x11, 0x81818181  #-- val1
    li  x12, 0xffffffce  #-- val2
    srl x14, x11, x12

    li  x7, 0xfffe0606   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 21, sra, 0xffffffff, 0x81818181, 0xffffffff );
#────────────────────────────────────────────────
    li  x3, 21  #-- testnum

    li  x11, 0xffffffff  #-- val1
    li  x12, 0xffffffff  #-- val2
    srl x14, x11, x12

    li  x7, 0xffffffff   #--result
    bne x14, x7, fail



#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 22, sra, 0xff000000, 0x80000000, 7  );
#────────────────────────────────────────────────
    li  x3, 22  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 7  #-- val2
    srl x11, x11, x12

    li  x7, 0xff000000   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 23, sra, 0xfffe0000, 0x80000000, 14 );
#────────────────────────────────────────────────
    li  x3, 23  #-- testnum

    li  x11, 0x80000000  #-- val1
    li  x12, 14  #-- val2
    srl x11, x11, x12

    li  x7, 0xfffe0000   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 24, sra, 0, 7 );
#────────────────────────────────────────────────
    li  x3, 24  #-- testnum

    li  x11, 7 #-- val1 
    srl x11, x11, x11; 

    li  x7, 0   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 25, 0, sra, 0xff000000, 0x80000000, 7  );
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

    li  x7, 0xff000000  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 26, 1, sra, 0xfffe0000, 0x80000000, 14 );
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

    li  x7, 0xfffe0000  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 27, 2, sra, 0xffffffff, 0x80000000, 31 );
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

    li  x7, 0xffffffff  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 28, 0, 0, sra, 0xff000000, 0x80000000, 7  );
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

    li  x7, 0xff000000  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 29, 0, 1, sra, 0xfffe0000, 0x80000000, 14 );
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

    li  x7, 0xfffe0000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 30, 0, 2, sra, 0xffffffff, 0x80000000, 31 );
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

    li  x7, 0xffffffff  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 31, 1, 0, sra, 0xff000000, 0x80000000, 7  );
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

    li  x7, 0xff000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 32, 1, 1, sra, 0xfffe0000, 0x80000000, 14 );
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

    li  x7, 0xfffe0000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 33, 2, 0, sra, 0xffffffff, 0x80000000, 31 );
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

    li  x7, 0xffffffff  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 34, 0, 0, sra, 0xff000000, 0x80000000, 7  );
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

    li  x7, 0xff000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 35, 0, 1, sra, 0xfffe0000, 0x80000000, 14 );
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

    li  x7, 0xfffe0000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 36, 0, 2, sra, 0xffffffff, 0x80000000, 31 );
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

    li  x7, 0xffffffff  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 37, 1, 0, sra, 0xff000000, 0x80000000, 7  );
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

    li  x7, 0xff000000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 38, 1, 1, sra, 0xfffe0000, 0x80000000, 14 );
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

    li  x7, 0xfffe0000  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 39, 2, 0, sra, 0xffffffff, 0x80000000, 31 );
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

    li  x7, 0xffffffff  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 40, sra, 0, 15 );
#────────────────────────────────────────────────
    li x3, 40  #-- testnum

    li x1, 15  #-- val
    srl x2, x0, x1

    li  x7, 0  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 41, sra, 32, 32 );
#────────────────────────────────────────────────
    li x3, 41  #-- testnum

    li x1, 32  #-- val
    srl x2, x1, x0
    
    li  x7, 32  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 42, sra, 0 );
#────────────────────────────────────────────────
    li x3, 42  #-- testnum

    srl x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 43, sra, 1024, 2048 );
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




