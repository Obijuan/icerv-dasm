#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE AND
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0xff00ff00  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    and x14, x11, x12

    li  x7, 0x0f000f00   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x0ff00ff0  #-- val1
    li  x12, 0xf0f0f0f0  #-- val2
    and x14, x11, x12

    li  x7, 0x00f000f0   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x00ff00ff  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    and x14, x11, x12

    li  x7, 0x000f000f  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5, and, 0xf000f000, 0xf00ff00f, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0xf00ff00f  #-- val1
    li  x12, 0xf0f0f0f0  #-- val2
    and x14, x11, x12

    li  x7, 0xf000f000   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 6, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0xff00ff00  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    and x11, x11, x12

    li  x7, 0x0f000f00   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 7, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0x0ff00ff0  #-- val1
    li  x12, 0xf0f0f0f0  #-- val2
    and x11, x11, x12

    li  x7, 0x00f000f0   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 8, and, 0xff00ff00, 0xff00ff00 );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0xff00ff00 #-- val1 
    and x11, x11, x11; 

    li  x7, 0xff00ff00   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 9,  0, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 9 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00  #-- val1
    li  x2, 0x0f0f0f0f  #-- val2
    and x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x0f000f00  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 10, 1, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 10 #-- testnum

    li  x4, 0
1:  li  x1, 0x0ff00ff0  #-- val1
    li  x2, 0xf0f0f0f0  #-- val2
    and x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x00f000f0  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 11, 2, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 11 #-- testnum

    li  x4, 0
1:  li  x1, 0x00ff00ff  #-- val1
    li  x2, 0x0f0f0f0f  #-- val2
    and x14, x1, x2
    nop   ## nop_cycles
    nop
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x000f000f  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 12, 0, 0, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 12 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00 #-- val1
               ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
               ## src2_nops 
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0f000f00  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 13, 0, 1, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 13 #-- testnum

    li  x4, 0
1:  li  x1, 0x0ff00ff0 #-- val1
               ## src1_nops 
    li  x2, 0xf0f0f0f0 #-- val2 
    nop        ## src2_nops 
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00f000f0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 14, 0, 2, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 14 #-- testnum

    li  x4, 0
1:  li  x1, 0x00ff00ff #-- val1
               ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
    nop        ## src2_nops 
    nop
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x000f000f  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 15, 1, 0, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 15 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00 #-- val1
    nop           ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
                ## src2_nops 
    
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0f000f00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 16, 1, 1, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 16 #-- testnum

    li  x4, 0
1:  li  x1, 0x0ff00ff0 #-- val1
    nop        ## src1_nops 
    li  x2, 0xf0f0f0f0 #-- val2 
    nop        ## src2_nops 
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00f000f0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 17, 2, 0, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 17 #-- testnum

    li  x4, 0
1:  li  x1, 0x00ff00ff #-- val1
    nop        ## src1_nops 
    nop
    li  x2, 0x0f0f0f0f #-- val2 
               ## src2_nops 
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x000f000f  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 18, 0, 0, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 18 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00 #-- val1
               ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
               ## src2_nops 
    and x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0f000f00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 19, 0, 1, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 19  #-- testnum

    li  x4, 0
1:  li  x2, 0xf0f0f0f0  #-- val2
                ## src1_nops 
     
    li x1, 0x0ff00ff0  #-- val1
    nop        ## src2_nops 
    and x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00f000f0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 20, 0, 2, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 20  #-- testnum

    li  x4, 0
1:  li  x2, 0x0f0f0f0f  #-- val2
              ## src1_nops 
     
    li x1,  0x00ff00ff  #-- val1
    nop           ## src2_nops 
    nop
    and x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x000f000f  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 21, 1, 0, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 21  #-- testnum

    li  x4, 0
1:  li  x2, 0x0f0f0f0f  #-- val2
    nop         ## src1_nops 
    li x1, 0xff00ff00  #-- val1
               ## src2_nops 
    and x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0f000f00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 22, 1, 1, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 22  #-- testnum

    li  x4, 0
1:  li  x2, 0xf0f0f0f0  #-- val2
    nop         ## src1_nops 
    li x1, 0x0ff00ff0  #-- val1
               ## src2_nops 
    and x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x00f000f0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 23, 2, 0, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 23  #-- testnum

    li  x4, 0
1:  li  x2, 0x0f0f0f0f  #-- val2
    nop         ## src1_nops 
    nop
    li x1, 0x00ff00ff  #-- val1
               ## src2_nops 
    and x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x000f000f  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 24, and, 0, 0xff00ff00 );
#────────────────────────────────────────────────
    li x3, 34  #-- testnum

    li x1, 0xff00ff00  #-- val
    and x2, x0, x1

    li  x7, 0x00000000  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 25, and, 0, 0x00ff00ff );
#────────────────────────────────────────────────
    li x3, 35  #-- testnum

    li x1, 0x00ff00ff  #-- val
    and x2, x1, x0
    
    li  x7, 0x00000000  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 26, and, 0 );
#────────────────────────────────────────────────
    li x3, 36  #-- testnum

    and x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 27, and, 0x11111111, 0x22222222 );
#────────────────────────────────────────────────
    li x3, 37  #-- testnum

    li x1, 0x11111111  #-- val1
    li x2, 0x22222222  #-- val2
    and x0, x1, x2
    
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




