#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SH
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 2, lh, sh,          0x000000aa, 0, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    la x2, tdat #-- base
    li x1, 0x000000aa  #-- result
    sh x1, 0(x2)       #-- offset
    lh x14, 0(x2)

    li  x7, 0x000000aa  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 3, lh, sh,           0xffffaa00, 2, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    la x2, tdat #-- base
    li x1, 0xffffaa00  #-- result
    sh x1,  2(x2)       #-- offset
    lh x14, 2(x2)      #-- offset

    li  x7, 0xffffaa00  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 4, lw, sh, 0xbeef0aa0, 4, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    la x2, tdat #-- base
    li x1, 0xbeef0aa0  #-- result
    sh x1,  4(x2)      #-- offset
    lw x14, 4(x2)      #-- offset

    li  x7, 0xbeef0aa0  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 5, lh, sh, 0xffffa00a, 6, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    la x2, tdat #-- base
    li x1, 0xffffa00a  #-- result
    sh x1,  6(x2)      #-- offset
    lh x14, 6(x2)      #-- offset

    li  x7, 0xffffa00a  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 6, lh, sh, 0x000000aa, -6, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0x000000aa  #-- result
    sh x1,  -6(x2)      #-- offset
    lh x14, -6(x2)      #-- offset

    li  x7, 0x000000aa  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 7, lh, sh, 0xffffaa00, -4, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0xffffaa00  #-- result
    sh x1,  -4(x2)      #-- offset
    lh x14, -4(x2)      #-- offset

    li  x7, 0xffffaa00  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 8, lh, sh, 0x00000aa0, -2, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0x00000aa0  #-- result
    sh x1,  -2(x2)      #-- offset
    lh x14, -2(x2)      #-- offset

    li  x7, 0x00000aa0  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 9, lh, sh, 0xffffa00a, 0,  tdat8 );
#───────────────────────────────────────────────────────────────────────
    li x3, 9  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0xffffa00a  #-- result
    sh x1,  0(x2)      #-- offset
    lh x14, 0(x2)      #-- offset

    li  x7, 0xffffa00a  #-- result
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────
#   TEST_CASE( 10, x5, 0x5678, 
#     la  x1, tdat9
#     li  x2, 0x12345678
#     addi x4, x1, -32
#     sh x2, 32(x4)
#     lh x5, 0(x1)
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 10  #-- testnum

    la  x1, tdat9 
    li  x2, 0x12345678 
    addi x4, x1, -32 
    sh x2, 32(x4) 
    lh x5, 0(x1) 

    li  x7, 0x5678      #-- result
    bne x5, x7, fail  #-- testreg

#───────────────────────────────────────────────────────────────────────
#         testnum, restreg, result
#   TEST_CASE( 11, x5, 0x3098, 
#     la  x1, tdat9
#     li  x2, 0x00003098
#     addi x1, x1, -5
#     sh x2, 7(x1)
#     la  x4, tdat10
#     lh x5, 0(x4)
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 11  #-- testnum

    la  x1, tdat9
    li  x2, 0x00003098
    addi x1, x1, -5
    sh x2, 7(x1)
    la  x4, tdat10
    lh x5, 0(x4)

    li  x7, 0x00003098      #-- result
    bne x5, x7, fail  #-- testreg

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 12, 0, 0, lh, sh,              0xffffccdd, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 12  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffccdd  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    sh x13, 0(x12)  #-- offset
    lh x14, 0(x12)  #-- offset
    li  x7, 0xffffccdd
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 13, 0, 1, lh, sh,              0xffffbccd, 2,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 13  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffbccd  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    sh x13, 2(x12)  #-- offset
    lh x14, 2(x12)  #-- offset
    li x7, 0xffffbccd #-- result
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 14, 0, 2, lh, sh,               0xffffbbcc, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 14  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffbbcc  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
    nop          ## src2_nops
    nop
    sh x13, 4(x12)  #-- offset
    lh x14, 4(x12)  #-- offset
    li x7, 0xffffbbcc
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 15, 1, 0, lh, sh,              0xffffabbc, 6, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 15  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffabbc  #-- result
    nop         ## src1_nops
    la  x12, tdat  #-- base
                 ## src2_nops
    
    sh x13, 6(x12)  #-- offset
    lh x14, 6(x12)  #-- offset
    li x7, 0xffffabbc
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 16, 1, 1, lh, sh,               0xffffaabb, 8, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 16  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffaabb  #-- result
    nop          ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    nop
    sh x13, 8(x12)  #-- offset
    lh x14, 8(x12)  #-- offset
    li x7, 0xffffaabb
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 17, 2, 0, lh, sh, 0xffffdaab, 10, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 17  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffdaab  #-- result
    nop          ## src1_nops
    nop
    la  x12, tdat  #-- base
               ## src2_nops
     
    sh x13, 10(x12)  #-- offset
    lh x14, 10(x12)  #-- offset
    li x7, 0xffffdaab
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 18, 0, 0, lh, sh,                   0x2233, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 18  #-- testnum
    li  x4, 0
1:  li  x13, 0x2233  #-- result
               ## src1_nops
     
    la  x12, tdat  #-- base
               ## src2_nops
     
    sh x13, 0(x12)  #-- offset
    lh x14, 0(x12)  #-- offset
    li x7, 0x2233
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 


#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 19, 0, 1, lh, sh,                   0x1223, 2,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 19  #-- testnum
    li  x4, 0
1:  li  x13, 0x1223  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sh x13, 2(x12)  #-- offset
    lh x14, 2(x12)  #-- offset
    li x7, 0x1223
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 20, 0, 2, lh, sh,                   0x1122, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 20  #-- testnum
    li  x4, 0
1:  li  x13, 0x1122  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sh x13, 4(x12)  #-- offset
    lh x14, 4(x12)  #-- offset
    li x7, 0x1122
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 21, 1, 0, lh, sh,                  0x0112, 6, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 21  #-- testnum
    li  x4, 0
1:  li  x13, 0x0112  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sh x13, 6(x12)  #-- offset
    lh x14, 6(x12)  #-- offset
    li x7, 0x0112
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 22, 1, 1, lh, sh,                   0x0011, 8, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 22  #-- testnum
    li  x4, 0
1:  li  x13, 0x0011  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sh x13, 8(x12)  #-- offset
    lh x14, 8(x12)  #-- offset
    li x7, 0x0011
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC21_BYPASS( 23, 2, 0, lh, sh,                 0x3001, 10, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 23  #-- testnum
    li  x4, 0
1:  li  x13, 0x3001  #-- result
                   ## src1_nops
    la  x12, tdat  #-- base
    nop            ## src2_nops
     
    sh x13, 10(x12)  #-- offset
    lh x14, 10(x12)  #-- offset
    li x7, 0x3001
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
tdat1:  .half 0xbeef
tdat2:  .half 0xbeef
tdat3:  .half 0xbeef
tdat4:  .half 0xbeef
tdat5:  .half 0xbeef
tdat6:  .half 0xbeef
tdat7:  .half 0xbeef
tdat8:  .half 0xbeef
tdat9:  .half 0xbeef
tdat10: .half 0xbeef


#   #-------------------------------------------------------------
#   # Basic tests
#   #-------------------------------------------------------------


#   # Test with negative offset


#   # Test with a negative base



#   # Test with unaligned base



#   #-------------------------------------------------------------
#   # Bypassing tests
#   #-------------------------------------------------------------


