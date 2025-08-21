#────────────────────────────────────────────────
# PRUEBAS PARA LA INSTRUCCIONE JALR
# Ejemplo tomado de:
# https://github.com/riscv-software-src/riscv-tests/
#
#────────────────────────────────────────────────

#───────────────────────────────────────────────────────────────────────
# Test 2: Basic test
#───────────────────────────────────────────────────────────────────────
test_2:
  li x3, 2
  li t0, 0
  la t1, target_2

  jalr t0, t1, 0
linkaddr_2:
  j fail

target_2:
  la  t1, linkaddr_2
  bne t0, t1, fail

#───────────────────────────────────────────────────────────────────────
#    Test 3: Basic test2, rs = rd
#───────────────────────────────────────────────────────────────────────
test_3:
  li x3, 3
  la t0, target_3

  jalr t0, t0, 0
linkaddr_3:
  j fail

target_3:
  la  t1, linkaddr_3
  bne t0, t1, fail

#───────────────────────────────────────────────────────────────────────
#                testnum, nop_cycles, inst 
#   TEST_JALR_SRC1_BYPASS( 4, 0, jalr );
#───────────────────────────────────────────────────────────────────────
    li x3, 4
    li x4, 0
1:  la  x6, 2f
                ## nop_cycles
    jalr x13, x6, 0
    bne x0, x3, fail
2:  addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#                testnum, nop_cycles, inst 
#   TEST_JALR_SRC1_BYPASS( 5, 1, jalr );
#───────────────────────────────────────────────────────────────────────
    li x3, 5
    li x4, 0
1:  la  x6, 2f
    nop          ## nop_cycles
    jalr x13, x6, 0
    bne x0, x3, fail
2:  addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b


#───────────────────────────────────────────────────────────────────────
#                testnum, nop_cycles, inst 
#   TEST_JALR_SRC1_BYPASS( 6, 2, jalr );
#───────────────────────────────────────────────────────────────────────
    li x3, 6
    li x4, 0
1:  la  x6, 2f
    nop          ## nop_cycles
    nop
    jalr x13, x6, 0
    bne x0, x3, fail
2:  addi  x4, x4, 1
    li  x5, 2
    bne x4, x5, 1b

#───────────────────────────────────────────────────────────────────────
#   Test delay slot instructions not executed nor bypassed
#   testnum, testreg, correctval, code... 
#   TEST_CASE( 7, t0, 4, 
#     li  t0, 1
#     la  t1, 1f
#     jr  t1, -4
#     addi t0, t0, 1
#     addi t0, t0, 1
#     addi t0, t0, 1
#     addi t0, t0, 1
# 1:  addi t0, t0, 1
#     addi t0, t0, 1
#───────────────────────────────────────────────────────────────────────
  .option push
  .align 2
  .option norvc

    li x3, 7  #-- testnum

    li  t0, 1
    la  t1, 1f
    jr  t1, -4
    addi t0, t0, 1
    addi t0, t0, 1
    addi t0, t0, 1
    addi t0, t0, 1
1:  addi t0, t0, 1
    addi t0, t0, 1

    li  x7, 4   #-- Correctval
    bne t0, x7, fail;


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

