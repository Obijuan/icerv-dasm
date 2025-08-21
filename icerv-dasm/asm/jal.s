#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE JAL
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
# Test 2: Basic test
#───────────────────────────────────────────────────────────────────────
test_2:
  li  x3, 2
  li  ra, 0

  jal x4, target_2
linkaddr_2:
  nop
  nop

  j fail

target_2:
  la  x2, linkaddr_2
  bne x2, x4, fail


#───────────────────────────────────────────────────────────────────────
# Test delay slot instructions not executed nor bypassed
#───────────────────────────────────────────────────────────────────────
    li x3, 3

    li  ra, 1
    jal x0, 1f
    addi ra, ra, 1
    addi ra, ra, 1
    addi ra, ra, 1
    addi ra, ra, 1
 1: addi ra, ra, 1
    addi ra, ra, 1

    li  x7, 3  #-- Correctval
    bne ra, x7, fail;



#   TEST_CASE( 3, ra, 3, \
#     li  ra, 1; \
#     jal x0, 1f; \
#     addi ra, ra, 1; \
#     addi ra, ra, 1; \
#     addi ra, ra, 1; \
#     addi ra, ra, 1; \
# 1:  addi ra, ra, 1; \
#     addi ra, ra, 1; \
#   )





# #define TEST_CASE( testnum, testreg, correctval, code... ) \
# test_ ## testnum: \
#     li  TESTNUM, testnum; \
#     code; \
#     li  x7, MASK_XLEN(correctval); \
#     bne testreg, x7, fail;


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
  # Test delay slot instructions not executed nor bypassed
  #-------------------------------------------------------------

