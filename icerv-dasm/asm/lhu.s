#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCION LHU
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#

    .text
#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,  offset, base
#  TEST_LD_OP( 2, lh, 0x000000ff, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- Numero de test
    la  x2, tdat  #-- base
    lhu x14, 0(x2); #-- offset 
    li  x7, 0x000000ff #-- Resultado
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,  offset, base
#   TEST_LD_OP( 3, lhu, 0x0000ff00, 2,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- Numero de test
    la  x2, tdat  #-- base
    lhu x14, 2(x2); #-- offset 
    li  x7, 0x0000ff00 #-- Resultado
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
#   TEST_LD_OP( 4, lhu, 0x00000ff0, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- Numero de test
    la  x2, tdat  #-- base
    lhu x14, 4(x2); #-- offset 
    li  x7, 0x00000ff0 #-- Resultado
    bne x14, x7, fail;    

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
#   TEST_LD_OP( 5, lhu, 0x0000f00f, 6, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5  #-- Numero de test
    la  x2, tdat  #-- base
    lhu x14, 6(x2); #-- offset 
    li  x7, 0x0000f00f #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
#   TEST_LD_OP( 6, lhu, 0x000000ff, -6,  tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6  #-- Numero de test
    la  x2, tdat4  #-- base
    lhu x14, -6(x2); #-- offset 
    li  x7, 0x000000ff #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
#   TEST_LD_OP( 7, lhu, 0x0000ff00, -4,  tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7  #-- Numero de test
    la  x2, tdat4  #-- base
    lhu x14, -4(x2); #-- offset 
    li  x7, 0x0000ff00 #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
#   TEST_LD_OP( 8, lhu, 0x00000ff0, -2,  tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 8  #-- Numero de test
    la  x2, tdat4  #-- base
    lhu x14, -2(x2); #-- offset 
    li  x7, 0x00000ff0 #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
#   TEST_LD_OP( 9, lhu, 0x0000f00f,  0, tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 9  #-- Numero de test
    la  x2, tdat4  #-- base
    lhu x14, 0(x2); #-- offset 
    li  x7, 0x0000f00f #-- Resultado
    bne x14, x7, fail; 

#───────────────────────────────────────────────────────────────────────
#         testnum  testreg,  result, code 
#   TEST_CASE( 10, x5, 0x00000000000000ff, \
#     la  x1, tdat; \
#     addi x1, x1, -32; \
#     lhu x5, 32(x1); \
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 10  #-- testnum

    la x1, tdat; #-- code
    addi x1, x1, -32; 
    lhu x5, 32(x1); 

    li  x7, 0x000000ff  #-- result
    bne x5, x7, fail    #-- testreg

#───────────────────────────────────────────────────────────────────────
#         testnum  testreg,  result, code 
#   TEST_CASE( 11, x5, 0x0000ff00, \
#     la  x1, tdat; \
#     addi x1, x1, -5; \
#     lhu x5, 7(x1); \
#   )
#───────────────────────────────────────────────────────────────────────
    li x3, 11  #-- testnum

    la  x1, tdat;   #-- code
    addi x1, x1, -5; 
    lhu x5, 7(x1); 

    li  x7, 0x0000ff00  #-- result
    bne x5, x7, fail    #-- testreg

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
#   TEST_LD_DEST_BYPASS( 12, 0, lhu, 0x00000ff0, 2, tdat2 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 12  #-- testnum
    li  x4, 0 
1:  la  x13, tdat2  #-- base
    lhu x14, 2(x13) #-- inst / offset
          ## nop_cycles 
    addi  x6, x14, 0
    li  x7, 0x00000ff0  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
#   TEST_LD_DEST_BYPASS( 13, 1, lhu, 0x0000f00f, 2, tdat3 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 13  #-- testnum
    li  x4, 0 
1:  la  x13, tdat3  #-- base
    lhu x14, 2(x13) #-- inst / offset
    nop      ## nop_cycles 
    addi  x6, x14, 0
    li  x7, 0x0000f00f  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
#   TEST_LD_DEST_BYPASS( 14, 2, lhu, 0x0000ff00, 2, tdat1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 14  #-- testnum
    li  x4, 0 
1:  la  x13, tdat1  #-- base
    lhu x14, 2(x13) #-- inst / offset
    nop      ## nop_cycles
    nop 
    addi  x6, x14, 0
    li  x7, 0x0000ff00  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
#   TEST_LD_SRC1_BYPASS( 15, 0, lhu, 0x00000ff0, 2, tdat2 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 15  #-- testnum
    li  x4, 0 
1:  la  x13, tdat2  #-- base
    lhu x14, 2(x13) #-- inst / offset
         ## nop_cycles
    
    addi  x6, x14, 0
    li  x7, 0x00000ff0  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
#   TEST_LD_SRC1_BYPASS( 16, 1, lhu, 0x0000f00f, 2, tdat3 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 16   #-- testnum
    li  x4, 0
1:  la  x13, tdat3  #-- base
    nop   ## nop_cycles 
    lhu x14, 2(x13);  #-- inst / offset
    li  x7, 0x0000f00f  #-- result
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
#   TEST_LD_SRC1_BYPASS( 17, 2, lhu, 0x000ff00, 2, tdat1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 17   #-- testnum
    li  x4, 0
1:  la  x13, tdat1  #-- base
    nop   ## nop_cycles
    nop 
    lhu x14, 2(x13);  #-- inst / offset
    li  x7, 0x0000ff00  #-- result
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#         testnum  testreg,  result, code 
#  TEST_CASE( 18, x2, 2, \
#    la  x5, tdat; \
#    lhu  x2, 0(x5); \
#    li  x2, 2; \
#  )
#───────────────────────────────────────────────────────────────────────
    li x3, 18  #-- testnum

    la  x5, tdat; #-- code
    lhu  x2, 0(x5) 
    li  x2, 2

    li  x7, 2  #-- result
    bne x2, x7, fail    #-- testreg

#───────────────────────────────────────────────────────────────────────
#  TEST_CASE( 19, x2, 2, \
#    la  x5, tdat; \
#    lhu  x2, 0(x5); \
#    nop; \
#    li  x2, 2; \
#  )
#───────────────────────────────────────────────────────────────────────
    li x3, 19  #-- testnum

    la  x5, tdat
    lhu  x2, 0(x5)
    nop
    li  x2, 2

    li  x7, 2  #-- result
    bne x2, x7, fail    #-- testreg



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


#───────────────────────────────────────────────────────────────────────
#   DATOS
#───────────────────────────────────────────────────────────────────────
#-- NOTA: No se pone .data para que los datos se incluyan
#-- justo a continuacion de las instrucciones, sin necesidad
#-- de crear un linker script 

#   TEST_DATA
    .data
tdat:
tdat1:  .half 0x00ff
tdat2:  .half 0xff00
tdat3:  .half 0x0ff0
tdat4:  .half 0xf00f




