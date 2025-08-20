#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SW
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 2, lw, sw,            0x00aa00aa, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    la x2, tdat #-- base
    li x1, 0x00aa00aa  #-- result
    sw x1,  0(x2)       #-- offset
    lw x14, 0(x2)

    li  x7, 0x00aa00aa  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 3, lw, sw, 0xaa00aa00, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    la x2, tdat #-- base
    li x1, 0xaa00aa00  #-- result
    sw x1,  4(x2)       #-- offset
    lw x14, 4(x2)      #-- offset

    li  x7, 0xaa00aa00  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 4, lw, sw,           0x0aa00aa0, 8,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    la x2, tdat #-- base
    li x1, 0x0aa00aa0  #-- result
    sw x1,  8(x2)      #-- offset
    lw x14, 8(x2)      #-- offset

    li  x7, 0x0aa00aa0  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 5, lw, sw,             0xa00aa00a, 12, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    la x2, tdat #-- base
    li x1, 0xa00aa00a  #-- result
    sw x1,  12(x2)      #-- offset
    lw x14, 12(x2)      #-- offset

    li  x7, 0xa00aa00a  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 6, lw, sw,            0x00aa00aa, -12, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0x00aa00aa  #-- result
    sw x1,  -12(x2)      #-- offset
    lw x14, -12(x2)      #-- offset

    li  x7, 0x00aa00aa  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 7, lw, sw,          0xaa00aa00, -8,  tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0xaa00aa00  #-- result
    sw x1,  -8(x2)      #-- offset
    lw x14, -8(x2)      #-- offset

    li  x7, 0xaa00aa00  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 8, lw, sw,          0x0aa00aa0, -4,  tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0x0aa00aa0  #-- result
    sw x1,  -4(x2)      #-- offset
    lw x14, -4(x2)      #-- offset

    li  x7, 0x0aa00aa0  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 9, lw, sw, 0xa00aa00a, 0,   tdat8 );
#───────────────────────────────────────────────────────────────────────
    li x3, 9  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0xa00aa00a  #-- result
    sw x1,  0(x2)      #-- offset
    lw x14, 0(x2)      #-- offset

    li  x7, 0xa00aa00a  #-- result
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────
#   TEST_CASE( 10, x5, 0x12345678, 
#     la  x1, tdat9
#     li  x2, 0x12345678
#     addi x4, x1, -32
#     sw x2, 32(x4)
#     lw x5, 0(x1)
#   )

#───────────────────────────────────────────────────────────────────────
    li x3, 10  #-- testnum

    la  x1, tdat9 
    li  x2, 0x12345678 
    addi x4, x1, -32 
    sw x2, 32(x4) 
    lw x5, 0(x1) 

    li  x7, 0x12345678      #-- result
    bne x5, x7, fail  #-- testreg

#───────────────────────────────────────────────────────────────────────
#   TEST_CASE( 11, x5, 0x58213098, \
#     la  x1, tdat9; \
#     li  x2, 0x58213098; \
#     addi x1, x1, -3; \
#     sw x2, 7(x1); \
#     la  x4, tdat10; \
#     lw x5, 0(x4); \
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 11  #-- testnum

    la  x1, tdat9
    li  x2, 0x58213098
    addi x1, x1, -3
    sw x2, 7(x1)
    la  x4, tdat10
    lw x5, 0(x4)

    li  x7, 0x58213098      #-- result
    bne x5, x7, fail  #-- testreg

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 12, 0, 0, lw, sw, 0xaabbccdd, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 12  #-- testnum
    li  x4, 0
1:  li  x13, 0xaabbccdd  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    sw x13, 0(x12)  #-- offset
    lw x14, 0(x12)  #-- offset
    li  x7, 0xaabbccdd
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 13, 0, 1, lw, sw, 0xdaabbccd, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 13  #-- testnum
    li  x4, 0
1:  li  x13, 0xdaabbccd  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    sw x13, 4(x12)  #-- offset
    lw x14, 4(x12)  #-- offset
    li x7, 0xdaabbccd #-- result
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 14, 0, 2, lw, sw,              0xddaabbcc, 8,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 14  #-- testnum
    li  x4, 0
1:  li  x13, 0xddaabbcc  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
    nop          ## src2_nops
    nop
    sw x13, 8(x12)  #-- offset
    lw x14, 8(x12)  #-- offset
    li x7, 0xddaabbcc
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 15, 1, 0, lw, sw,               0xcddaabbc, 12, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 15  #-- testnum
    li  x4, 0
1:  li  x13, 0xcddaabbc  #-- result
    nop         ## src1_nops
    la  x12, tdat  #-- base
                 ## src2_nops
    
    sw x13, 12(x12)  #-- offset
    lw x14, 12(x12)  #-- offset
    li x7, 0xcddaabbc
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 16, 1, 1, lw, sw, 0xccddaabb, 16, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 16  #-- testnum
    li  x4, 0
1:  li  x13, 0xccddaabb  #-- result
    nop          ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    nop
    sw x13, 16(x12)  #-- offset
    lw x14, 16(x12)  #-- offset
    li x7, 0xccddaabb
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 17, 2, 0, lw, sw, 0xbccddaab, 20, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 17  #-- testnum
    li  x4, 0
1:  li  x13, 0xbccddaab  #-- result
    nop          ## src1_nops
    nop
    la  x12, tdat  #-- base
               ## src2_nops
     
    sw x13, 20(x12)  #-- offset
    lw x14, 20(x12)  #-- offset
    li x7, 0xbccddaab
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 18, 0, 0, lw, sw, 0x00112233, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 18  #-- testnum
    li  x4, 0
1:  li  x13, 0x00112233  #-- result
               ## src1_nops
     
    la  x12, tdat  #-- base
               ## src2_nops
     
    sw x13, 0(x12)  #-- offset
    lw x14, 0(x12)  #-- offset
    li x7, 0x00112233
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 


#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 19, 0, 1, lw, sw, 0x30011223, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 19  #-- testnum
    li  x4, 0
1:  li  x13, 0x30011223  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sw x13, 4(x12)  #-- offset
    lw x14, 4(x12)  #-- offset
    li x7, 0x30011223
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 20, 0, 2, lw, sw, 0x33001122, 8,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 20  #-- testnum
    li  x4, 0
1:  li  x13, 0x33001122  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sw x13, 8(x12)  #-- offset
    lw x14, 8(x12)  #-- offset
    li x7, 0x33001122
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 21, 1, 0, lw, sw, 0x23300112, 12, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 21  #-- testnum
    li  x4, 0
1:  li  x13, 0x23300112  #-- result
    nop               ## src1_nops
    la  x12, tdat  #-- base
                 ## src2_nops
     
    sw x13, 12(x12)  #-- offset
    lw x14, 12(x12)  #-- offset
    li x7, 0x23300112
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 22, 1, 1, lw, sw, 0x22330011, 16, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 22  #-- testnum
    li  x4, 0
1:  li  x13, 0x22330011  #-- result
    nop               ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sw x13, 16(x12)  #-- offset
    lw x14, 16(x12)  #-- offset
    li x7, 0x22330011
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 23, 2, 0, lw, sw, 0x12233001, 20, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 23  #-- testnum
    li  x4, 0
1:  li  x13, 0x12233001  #-- result
    nop               ## src1_nops
    nop
    la  x12, tdat  #-- base
                 ## src2_nops
     
    sw x13, 20(x12)  #-- offset
    lw x14, 20(x12)  #-- offset
    li x7, 0x12233001
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

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



    .data
tdat:
tdat1:  .word 0xdeadbeef
tdat2:  .word 0xdeadbeef
tdat3:  .word 0xdeadbeef
tdat4:  .word 0xdeadbeef
tdat5:  .word 0xdeadbeef
tdat6:  .word 0xdeadbeef
tdat7:  .word 0xdeadbeef
tdat8:  .word 0xdeadbeef
tdat9:  .word 0xdeadbeef
tdat10: .word 0xdeadbeef


