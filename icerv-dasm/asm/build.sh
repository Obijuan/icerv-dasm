#!/bin/bash

riscv64-unknown-elf-as -march=rv32i addi-test2.s -o addi-test2.o
riscv64-unknown-elf-objcopy -O binary addi-test2.o addi-test2.bin

