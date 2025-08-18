#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCION LW
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#

    .text
#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,  offset, base
# TEST_LD_OP( 2, lw, 0x00ff00ff, 0,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 2  #-- Numero de test
    la  x2, tdat  #-- base
    lw x14, 0(x2); #-- offset 
    li  x7, 0x00ff00ff #-- Resultado
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,  offset, base
# TEST_LD_OP( 3, lw, 0xff00ff00, 4,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 3  #-- Numero de test
    la  x2, tdat  #-- base
    lw x14, 4(x2); #-- offset 
    li  x7, 0xff00ff00 #-- Resultado
    bne x14, x7, fail;

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
# TEST_LD_OP( 4, lw, 0x0ff00ff0, 8,  tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 4  #-- Numero de test
    la  x2, tdat  #-- base
    lw x14, 8(x2); #-- offset 
    li  x7, 0x0ff00ff0 #-- Resultado
    bne x14, x7, fail;    

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
# TEST_LD_OP( 5, lw, 0xf00ff00f, 12, tdat );
#───────────────────────────────────────────────────────────────────────
    li  x3, 5  #-- Numero de test
    la  x2, tdat  #-- base
    lw x14, 12(x2); #-- offset 
    li  x7, 0xf00ff00f #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
# TEST_LD_OP( 6, lw, 0x00ff00ff, -12, tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 6  #-- Numero de test
    la  x2, tdat4  #-- base
    lw x14, -12(x2); #-- offset 
    li  x7, 0x00ff00ff #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
# TEST_LD_OP( 7, lw, 0xff00ff00, -8,  tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 7  #-- Numero de test
    la  x2, tdat4  #-- base
    lw x14, -8(x2); #-- offset 
    li  x7, 0xff00ff00 #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
# TEST_LD_OP( 8, lw, 0x0ff00ff0, -4,  tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 8  #-- Numero de test
    la  x2, tdat4  #-- base
    lw x14, -4(x2); #-- offset 
    li  x7, 0x0ff00ff0 #-- Resultado
    bne x14, x7, fail;  

#───────────────────────────────────────────────────────────────────────
#         testnum, inst, result,       offset, base
# TEST_LD_OP( 9, lw, 0xf00ff00f, 0,   tdat4 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 9  #-- Numero de test
    la  x2, tdat4  #-- base
    lw x14, 0(x2); #-- offset 
    li  x7, 0xf00ff00f #-- Resultado
    bne x14, x7, fail; 

#───────────────────────────────────────────────────────────────────────
#         testnum  testreg,  result, code 
#  TEST_CASE( 10, x5, 0x00ff00ff, 
#    la  x1, tdat
#    addi x1, x1, -32
#    lw x5, 32(x1)
#  )
#───────────────────────────────────────────────────────────────────────
    li x3, 10  #-- testnum

    la x1, tdat; #-- code
    addi x1, x1, -32; 
    lw x5, 32(x1); 

    li  x7, 0x00ff00ff  #-- result
    bne x5, x7, fail    #-- testreg

#───────────────────────────────────────────────────────────────────────
#         testnum  testreg,  result, code 
#  TEST_CASE( 11, x5, 0xff00ff00,
#    la  x1, tdat
#    addi x1, x1, -3
#    lw x5, 7(x1)
#  )
#───────────────────────────────────────────────────────────────────────
    li x3, 11  #-- testnum

    la  x1, tdat;   #-- code
    addi x1, x1, -3; 
    lw x5, 7(x1); 

    li  x7, 0xff00ff00  #-- result
    bne x5, x7, fail    #-- testreg

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
# TEST_LD_DEST_BYPASS( 12, 0, lw, 0x0ff00ff0, 4, tdat2 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 12  #-- testnum
    li  x4, 0 
1:  la  x13, tdat2  #-- base
    lw x14, 4(x13) #-- inst / offset
          ## nop_cycles 
    addi  x6, x14, 0
    li  x7, 0x0ff00ff0  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
# TEST_LD_DEST_BYPASS( 13, 1, lw, 0xf00ff00f, 4, tdat3 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 13  #-- testnum
    li  x4, 0 
1:  la  x13, tdat3  #-- base
    lw x14, 4(x13) #-- inst / offset
    nop      ## nop_cycles 
    addi  x6, x14, 0
    li  x7, 0xf00ff00f  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
# TEST_LD_DEST_BYPASS( 14, 2, lw, 0xff00ff00, 4, tdat1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 14  #-- testnum
    li  x4, 0 
1:  la  x13, tdat1  #-- base
    lw x14, 4(x13) #-- inst / offset
    nop      ## nop_cycles
    nop 
    addi  x6, x14, 0
    li  x7, 0xff00ff00  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
# TEST_LD_SRC1_BYPASS( 15, 0, lw, 0x0ff00ff0, 4, tdat2 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 15  #-- testnum
    li  x4, 0 
1:  la  x13, tdat2  #-- base
    lw x14, 4(x13) #-- inst / offset
         ## nop_cycles
    
    addi  x6, x14, 0
    li  x7, 0x0ff00ff0  #-- results 
    bne x6, x7, fail
    addi  x4, x4, 1
    li x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
# TEST_LD_SRC1_BYPASS( 16, 1, lw, 0xf00ff00f, 4, tdat3 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 16   #-- testnum
    li  x4, 0
1:  la  x13, tdat3  #-- base
    nop   ## nop_cycles 
    lw x14, 4(x13);  #-- inst / offset
    li  x7, 0xf00ff00f  #-- result
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#          testnum  nop_cyles, inst,  result, offset, base 
# TEST_LD_SRC1_BYPASS( 17, 2, lw, 0xff00ff00, 4, tdat1 );
#───────────────────────────────────────────────────────────────────────
    li  x3, 17   #-- testnum
    li  x4, 0
1:  la  x13, tdat1  #-- base
    nop   ## nop_cycles
    nop 
    lw x14, 4(x13);  #-- inst / offset
    li  x7, 0xff00ff00  #-- result
    bne x14, x7, fail
    addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b 

#───────────────────────────────────────────────────────────────────────
#         testnum  testreg,  result, code 
#  TEST_CASE( 18, x2, 2,
#    la  x5, tdat
#    lw  x2, 0(x5)
#    li  x2, 2
#  )
#───────────────────────────────────────────────────────────────────────
    li x3, 18  #-- testnum

    la  x5, tdat; #-- code
    lw  x2, 0(x5) 
    li  x2, 2

    li  x7, 2  #-- result
    bne x2, x7, fail    #-- testreg

#───────────────────────────────────────────────────────────────────────
#  TEST_CASE( 19, x2, 2,
#    la  x5, tdat
#    lw  x2, 0(x5)
#    nop
#    li  x2, 2
#  )
#───────────────────────────────────────────────────────────────────────
    li x3, 19  #-- testnum

    la  x5, tdat
    lw  x2, 0(x5)
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
tdat1:  .word 0x00ff00ff
tdat2:  .word 0xff00ff00
tdat3:  .word 0x0ff00ff0
tdat4:  .word 0xf00ff00f

