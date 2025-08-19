#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE SB
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 2, lb, sb,             0xffffffaa, 0, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- testnum

    la x2, tdat #-- base
    li x1, 0xffffffaa  #-- result
    sb x1, 0(x2)       #-- offset
    lb x14, 0(x2)

    li  x7, 0xffffffaa  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 3, lb, sb, 0x00000000, 1, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- testnum

    la x2, tdat #-- base
    li x1, 0x00000000  #-- result
    sb x1, 1(x2)       #-- offset
    lb x14, 1(x2)      #-- offset

    li  x7, 0x00000000  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 4, lh, sb, 0xffffefa0, 2, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- testnum

    la x2, tdat #-- base
    li x1, 0xffffefa0  #-- result
    sb x1,  2(x2)      #-- offset
    lh x14, 2(x2)      #-- offset

    li  x7, 0xffffefa0  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 5, lb, sb, 0x0000000a, 3, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5  #-- testnum

    la x2, tdat #-- base
    li x1, 0x0000000a  #-- result
    sb x1,  3(x2)      #-- offset
    lb x14, 3(x2)      #-- offset

    li  x7, 0x0000000a  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 6, lb, sb, 0xffffffaa, -3, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0xffffffaa  #-- result
    sb x1,  -3(x2)      #-- offset
    lb x14, -3(x2)      #-- offset

    li  x7, 0xffffffaa  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 7, lb, sb, 0x00000000, -2, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0x00000000  #-- result
    sb x1,  -2(x2)      #-- offset
    lb x14, -2(x2)      #-- offset

    li  x7, 0x00000000  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 8, lb, sb, 0xffffffa0, -1, tdat8 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 8  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0xffffffa0  #-- result
    sb x1,  -1(x2)      #-- offset
    lb x14, -1(x2)      #-- offset

    li  x7, 0xffffffa0  #-- result
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#     testnum, load_inst, store_inst, result, offset, base
#   TEST_ST_OP( 9, lb, sb, 0x0000000a, 0,  tdat8 );
#───────────────────────────────────────────────────────────────────────
    li x3, 9  #-- testnum

    la x2, tdat8 #-- base
    li x1, 0x0000000a  #-- result
    sb x1,  0(x2)      #-- offset
    lb x14, 0(x2)      #-- offset

    li  x7, 0x0000000a  #-- result
    bne x14, x7, fail;


#───────────────────────────────────────────────────────────────────────
#         testnum, restreg, result
#   TEST_CASE( 10, x5, 0x78, 
#     la  x1, tdat9 
#     li  x2, 0x12345678 
#     addi x4, x1, -32 
#     sb x2, 32(x4) 
#     lb x5, 0(x1) 
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 10  #-- testnum

    la  x1, tdat9 
    li  x2, 0x12345678 
    addi x4, x1, -32 
    sb x2, 32(x4) 
    lb x5, 0(x1) 

    li  x7, 0x78      #-- result
    bne x5, x7, fail  #-- testreg

#───────────────────────────────────────────────────────────────────────
#         testnum, restreg, result
#   TEST_CASE( 11, x5, 0xffffff98, 
#     la  x1, tdat9
#     li  x2, 0x00003098
#     addi x1, x1, -6
#     sb x2, 7(x1)
#     la  x4, tdat10
#     lb x5, 0(x4)
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 11  #-- testnum

    la  x1, tdat9
    li  x2, 0xffffff98
    addi x1, x1, -6
    sb x2, 7(x1)
    la  x4, tdat10
    lb x5, 0(x4)

    li  x7, 0xffffff98      #-- result
    bne x5, x7, fail  #-- testreg

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 12, 0, 0, lb, sb,                0xffffffdd, 0, tdat )
#───────────────────────────────────────────────────────────────────────
    li x3, 12  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffffdd  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    sb x13, 0(x12)  #-- offset
    lb x14, 0(x12)  #-- offset
    li  x7, 0xffffffdd
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 13, 0, 1, lb, sb, 0xffffffcd, 1, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 13  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffffcd  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    sb x13, 1(x12)  #-- offset
    lb x14, 1(x12)  #-- offset
    li x7, 0xffffffcd #-- result
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 14, 0, 2, lb, sb,              0xffffffcc, 2, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 14  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffffcc  #-- result
              ## src1_nops
    la  x12, tdat  #-- base
    nop          ## src2_nops
    nop
    sb x13, 2(x12)  #-- offset
    lb x14, 2(x12)  #-- offset
    li x7, 0xffffffcc
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 15, 1, 0, lb, sb, 0xffffffbc, 3, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 15  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffffbc  #-- result
    nop         ## src1_nops
    la  x12, tdat  #-- base
                 ## src2_nops
    
    sb x13, 3(x12)  #-- offset
    lb x14, 3(x12)  #-- offset
    li x7, 0xffffffbc
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 16, 1, 1, lb, sb, 0xffffffbb, 4, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 16  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffffbb  #-- result
    nop          ## src1_nops
    la  x12, tdat  #-- base
              ## src2_nops
    nop
    sb x13, 4(x12)  #-- offset
    lb x14, 4(x12)  #-- offset
    li x7, 0xffffffbb
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#     testnum, src1_nops, src2_nops, load_inst, store_inst, result, offset, base ) 
#   TEST_ST_SRC12_BYPASS( 17, 2, 0, lb, sb, 0xffffffab, 5, tdat );
#───────────────────────────────────────────────────────────────────────
    li x3, 17  #-- testnum
    li  x4, 0
1:  li  x13, 0xffffffab  #-- result
    nop          ## src1_nops
    nop
    la  x12, tdat  #-- base
               ## src2_nops
     
    sb x13, 5(x12)  #-- offset
    lb x14, 5(x12)  #-- offset
    li x7, 0xffffffab
    bne x14, x7, fail
    addi x4, x4, 1
    li x5, 2
    bne x4, x5, 1b 



#   TEST_ST_SRC21_BYPASS( 18, 0, 0, lb, sb, 0x33, 0, tdat );
#   TEST_ST_SRC21_BYPASS( 19, 0, 1, lb, sb, 0x23, 1, tdat );
#   TEST_ST_SRC21_BYPASS( 20, 0, 2, lb, sb, 0x22, 2, tdat );
#   TEST_ST_SRC21_BYPASS( 21, 1, 0, lb, sb, 0x12, 3, tdat );
#   TEST_ST_SRC21_BYPASS( 22, 1, 1, lb, sb, 0x11, 4, tdat );
#   TEST_ST_SRC21_BYPASS( 23, 2, 0, lb, sb, 0x01, 5, tdat );

#   li a0, 0xef
#   la a1, tdat
#   sb a0, 3(a1)

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
tdat1:  .byte 0xef
tdat2:  .byte 0xef
tdat3:  .byte 0xef
tdat4:  .byte 0xef
tdat5:  .byte 0xef
tdat6:  .byte 0xef
tdat7:  .byte 0xef
tdat8:  .byte 0xef
tdat9:  .byte 0xef
tdat10: .byte 0xef
