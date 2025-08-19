#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE XOR
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 2, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    li  x11, 0xff00ff00  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    xor x14, x11, x12

    li  x7, 0xf00ff00f   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 3, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    li  x11, 0x0ff00ff0  #-- val1
    li  x12, 0xf0f0f0f0  #-- val2
    xor x14, x11, x12

    li  x7, 0xff00ff00   #--result
    bne x14, x7, fail


#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 4, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    li  x11, 0x00ff00ff  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    xor x14, x11, x12

    li  x7, 0x0ff00ff0  #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,  result,     val1,       val2
#   TEST_RR_OP( 5, xor, 0x00ff00ff, 0xf00ff00f, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    li  x11, 0xf00ff00f  #-- val1
    li  x12, 0xf0f0f0f0  #-- val2
    xor x14, x11, x12

    li  x7, 0x00ff00ff   #--result
    bne x14, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,       result, val1, val2
#   TEST_RR_SRC1_EQ_DEST( 6, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    li  x11, 0xff00ff00  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    xor x11, x11, x12

    li  x7, 0xf00ff00f   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#        testnum, inst,        result, val1, val2
#   TEST_RR_SRC2_EQ_DEST( 7, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    li  x11, 0xff00ff00  #-- val1
    li  x12, 0x0f0f0f0f  #-- val2
    xor x11, x11, x12

    li  x7, 0xf00ff00f   #--result
    bne x11, x7, fail

#────────────────────────────────────────────────
#                  testnum, inst, result, val1
#   TEST_RR_SRC12_EQ_DEST( 8, xor, 0x00000000, 0xff00ff00 );
#────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    li  x11, 0xff00ff00 #-- val1 
    xor x11, x11, x11; 

    li  x7, 0x00000000   #--result
    bne x11, x7, fail


#────────────────────────────────────────────────
#        testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 9,  0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 9 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00  #-- val1
    li  x2, 0x0f0f0f0f  #-- val2
    xor x14, x1, x2
       ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0xf00ff00f  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 10, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 10 #-- testnum

    li  x4, 0
1:  li  x1, 0x0ff00ff0  #-- val1
    li  x2, 0xf0f0f0f0  #-- val2
    xor x14, x1, x2
    nop   ## nop_cycles
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0xff00ff00  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#              testnum, nop_cycles, inst, result, val1, val2
#   TEST_RR_DEST_BYPASS( 11, 2, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 11 #-- testnum

    li  x4, 0
1:  li  x1, 0x00ff00ff  #-- val1
    li  x2, 0x0f0f0f0f  #-- val2
    xor x14, x1, x2
    nop   ## nop_cycles
    nop
    addi  x6, x14, 0
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

    li  x7, 0x0ff00ff0  #-- result
    bne x6, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 12, 0, 0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 12 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00 #-- val1
               ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
               ## src2_nops 
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xf00ff00f  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 13, 0, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 13 #-- testnum

    li  x4, 0
1:  li  x1, 0x0ff00ff0 #-- val1
               ## src1_nops 
    li  x2, 0xf0f0f0f0 #-- val2 
    nop        ## src2_nops 
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xff00ff00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 14, 0, 2, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 14 #-- testnum

    li  x4, 0
1:  li  x1, 0x00ff00ff #-- val1
               ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
    nop        ## src2_nops 
    nop
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0ff00ff0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 15, 1, 0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 15 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00 #-- val1
    nop           ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
                ## src2_nops 
    
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xf00ff00f  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 16, 1, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 16 #-- testnum

    li  x4, 0
1:  li  x1, 0x0ff00ff0 #-- val1
    nop        ## src1_nops 
    li  x2, 0xf0f0f0f0 #-- val2 
    nop        ## src2_nops 
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xff00ff00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC12_BYPASS( 17, 2, 0, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 17 #-- testnum

    li  x4, 0
1:  li  x1, 0x00ff00ff #-- val1
    nop        ## src1_nops 
    nop
    li  x2, 0x0f0f0f0f #-- val2 
               ## src2_nops 
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0ff00ff0  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 18, 0, 0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 28 #-- testnum

    li  x4, 0
1:  li  x1, 0xff00ff00 #-- val1
               ## src1_nops 
    li  x2, 0x0f0f0f0f #-- val2 
               ## src2_nops 
    xor x14, x1, x2
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xf00ff00f  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 19, 0, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 29  #-- testnum

    li  x4, 0
1:  li  x2, 0xf0f0f0f0  #-- val2
                ## src1_nops 
     
    li x1, 0x0ff00ff0  #-- val1
    nop        ## src2_nops 
    xor x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xff00ff00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 20, 0, 2, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 20  #-- testnum

    li  x4, 0
1:  li  x2, 0x0f0f0f0f  #-- val2
              ## src1_nops 
     
    li x1, 0x00ff00ff  #-- val1
    nop           ## src2_nops 
    nop
    xor x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0ff00ff0  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 21, 1, 0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 21  #-- testnum

    li  x4, 0
1:  li  x2, 0x0f0f0f0f  #-- val2
    nop         ## src1_nops 
    li x1, 0xff00ff00  #-- val1
               ## src2_nops 
    xor x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xf00ff00f  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 22, 1, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0 );
#────────────────────────────────────────────────
    li  x3, 22  #-- testnum

    li  x4, 0
1:  li  x2, 0xf0f0f0f0  #-- val2
    nop         ## src1_nops 
    li x1, 0x0ff00ff0  #-- val1
               ## src2_nops 
    xor x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0xff00ff00  #-- result
    bne x14, x7, fail;

#────────────────────────────────────────────────
#       testnum, src1_nops, src2_nops, inst, result, val1, val2
#   TEST_RR_SRC21_BYPASS( 23, 2, 0, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f );
#────────────────────────────────────────────────
    li  x3, 23  #-- testnum

    li  x4, 0
1:  li  x2, 0x0f0f0f0f  #-- val2
    nop         ## src1_nops 
    nop
    li x1, 0x00ff00ff  #-- val1
               ## src2_nops 
    xor x14, x1, x2
    addi x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

    li  x7, 0x0ff00ff0  #-- result
    bne x14, x7, fail;


#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC1( 24, xor, 0xff00ff00, 0xff00ff00 );
#────────────────────────────────────────────────
    li x3, 34  #-- testnum

    li x1, 0xff00ff00  #-- val
    xor x2, x0, x1

    li  x7, 0xff00ff00  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result, val
#   TEST_RR_ZEROSRC2( 25, xor, 0x00ff00ff, 0x00ff00ff );
#────────────────────────────────────────────────
    li x3, 35  #-- testnum

    li x1, 0x00ff00ff  #-- val
    xor x2, x1, x0
    
    li  x7, 0x00ff00ff  #-- Result
    bne x2, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, result
#   TEST_RR_ZEROSRC12( 26, xor, 0 );
#────────────────────────────────────────────────
    li x3, 36  #-- testnum

    xor x1, x0, x0
    
    li  x7, 0  #-- Result
    bne x1, x7, fail

#────────────────────────────────────────────────
#              testnum, inst, val1, val2 
#   TEST_RR_ZERODEST( 27, xor, 0x11111111, 0x22222222 );
#────────────────────────────────────────────────
    li x3, 37  #-- testnum

    li x1, 0x11111111  #-- val1
    li x2, 0x22222222  #-- val2
    xor x0, x1, x2
    
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


  #-------------------------------------------------------------
  # Logical tests
  #-------------------------------------------------------------


#   #-------------------------------------------------------------
#   # Source/Destination tests
#   #-------------------------------------------------------------


#   #-------------------------------------------------------------
#   # Bypassing tests
#   #-------------------------------------------------------------





