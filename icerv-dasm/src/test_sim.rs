//────────────────────────────────────────────────
//  TESTS PARA EL MODULO DE SIMULACION (SIM)
//────────────────────────────────────────────────

#[cfg(test)]
use icerv_dasm::sim::sim_test;

#[cfg(test)]
use icerv_dasm::instructionrv::InstructionRV;

#[cfg(test)]
use icerv_dasm::regs::Reg;

#[cfg(test)]
use icerv_dasm::memory::Memory;

#[cfg(test)]
use icerv_dasm::cpurv::Cpurv;

//──────────────────────────────────────────────────────────────────────────
//  TESTS DE LOS PROGRAMAS OFICIALES DE PRUEBAS DE RISCV
//  Tomados de # https://github.com/riscv-software-src/riscv-tests/
//──────────────────────────────────────────────────────────────────────────

#[test]
fn test_addi() 
{
    sim_test("asm/addi.bin", 210);
}

#[test]
fn test_slli() 
{
    sim_test("asm/slli.bin", 210);
}

#[test]
fn test_slti() 
{
    sim_test("asm/slti.bin", 210);
}

#[test]
fn test_sltiu() 
{
    sim_test("asm/sltiu.bin", 210);
}

#[test]
fn test_srli() 
{
    sim_test("asm/srli.bin", 220);
}

#[test]
fn test_srai() 
{
    sim_test("asm/srai.bin", 230);
}

#[test]
fn test_xori() 
{
    sim_test("asm/xori.bin", 170);
}

#[test]
fn test_ori() 
{
    sim_test("asm/ori.bin", 230);
}

#[test]
fn test_andi() 
{
    sim_test("asm/andi.bin", 230);
}

#[test]
fn test_lb() 
{
    sim_test("asm/lb.bin", 170);
}

#[test]
fn test_lbu() 
{
    sim_test("asm/lbu.bin", 185);
}

#[test]
fn test_lh() 
{
    sim_test("asm/lh.bin", 195);
}

#[test]
fn test_lhu() 
{
    sim_test("asm/lhu.bin", 205);
}

#[test]
fn test_lw() 
{
    sim_test("asm/lw.bin", 205);
}

#[test]
fn test_add() 
{
    sim_test("asm/add.bin", 425);
}

#[test]
fn test_sub() 
{
    sim_test("asm/sub.bin", 415);
}

#[test]
fn test_sll() 
{
    sim_test("asm/sll.bin", 450);
}

#[test]
fn test_slt() 
{
    sim_test("asm/slt.bin", 415);
}

#[test]
fn test_sltu() 
{
    sim_test("asm/sltu.bin", 415);
}

#[test]
fn test_xor() 
{
    sim_test("asm/xor.bin", 450);
}

#[test]
fn test_srl() 
{
    sim_test("asm/srl.bin", 470);
}

#[test]
fn test_or() 
{
    sim_test("asm/or.bin", 450);
}

#[test]
fn test_and() 
{
    sim_test("asm/and.bin", 445);
}


#[test]
fn test_sra() 
{
    sim_test("asm/sra.bin", 470);
}

#[test]
fn test_sb() 
{
    sim_test("asm/sb.bin", 345);
}

#[test]
fn test_sh() 
{
    sim_test("asm/sh.bin", 400);
}

#[test]
fn test_sw() 
{
    sim_test("asm/sw.bin", 415);
}

#[test]
fn test_beq() 
{
    sim_test("asm/beq.bin", 255);
}

#[test]
fn test_bne() 
{
    sim_test("asm/bne.bin", 255);
}

#[test]
fn test_blt() 
{
    sim_test("asm/blt.bin", 260);
}

#[test]
fn test_bge() 
{
    sim_test("asm/bge.bin", 300);
}

#[test]
fn test_bltu() 
{
    sim_test("asm/bltu.bin", 280);
}

#[test]
fn test_bgeu() 
{
    sim_test("asm/bgeu.bin", 330);
}

#[test]
fn test_lui() 
{
    sim_test("asm/lui.bin", 25);
}

#[test]
fn test_auipc() 
{
    sim_test("asm/auipc.bin", 20);
}

#[test]
fn test_jal() 
{
    sim_test("asm/jal.bin", 15);
}

#[test]
fn test_jalr() 
{
    sim_test("asm/jalr.bin", 80);
}

#[test]
fn test_ecall() 
{
    sim_test("asm/ecall.bin", 5);
}

#[test]
fn test_addi_1()
{
    //-- Programa de test a ejecutar
    let prog: Vec<u32> = vec![
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }
            .to_mcode(), 
        InstructionRV::Addi { rd: Reg::X2, rs1: Reg::X0, imm: 2 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: 3 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X4, rs1: Reg::X0, imm: 4 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X5, rs1: Reg::X0, imm: 5 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X6, rs1: Reg::X0, imm: 6 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X7, rs1: Reg::X0, imm: 7 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X8, rs1: Reg::X0, imm: 8 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X9, rs1: Reg::X0, imm: 9 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X10, rs1: Reg::X0, imm: 10 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X11, rs1: Reg::X0, imm: 11 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X12, rs1: Reg::X0, imm: 12 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X13, rs1: Reg::X0, imm: 13 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X14, rs1: Reg::X0, imm: 14 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X15, rs1: Reg::X0, imm: 15 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X16, rs1: Reg::X0, imm: 16 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X0, imm: 17 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X18, rs1: Reg::X0, imm: 18 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X19, rs1: Reg::X0, imm: 19 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X20, rs1: Reg::X0, imm: 20 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X21, rs1: Reg::X0, imm: 21 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X22, rs1: Reg::X0, imm: 22 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X23, rs1: Reg::X0, imm: 23 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X24, rs1: Reg::X0, imm: 24 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X25, rs1: Reg::X0, imm: 25 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X26, rs1: Reg::X0, imm: 26 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X27, rs1: Reg::X0, imm: 27 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X28, rs1: Reg::X0, imm: 28 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X29, rs1: Reg::X0, imm: 29 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X30, rs1: Reg::X0, imm: 30 }
            .to_mcode(),
        InstructionRV::Addi { rd: Reg::X31, rs1: Reg::X0, imm: 31 }
            .to_mcode(),
    ];

    //-- Crear la memoria con el programa
    let mem = Memory::from_u32(prog);

    //-- Crear CPU
    let mut cpu = Cpurv::new(mem);

    //-- Ejecutar el programa!
    cpu.run(100);

    //-- Mostrar estado final
    cpu.show();

    //-- Comprobar el estado final
    assert_eq!(cpu.x1, 1);
    assert_eq!(cpu.x2, 2);
    assert_eq!(cpu.x3, 3);
    assert_eq!(cpu.x4, 4);
    assert_eq!(cpu.x5, 5);
    assert_eq!(cpu.x6, 6);
    assert_eq!(cpu.x7, 7);
    assert_eq!(cpu.x8, 8);
    assert_eq!(cpu.x9, 9);
    assert_eq!(cpu.x10, 10);
    assert_eq!(cpu.x11, 11);
    assert_eq!(cpu.x12, 12);
    assert_eq!(cpu.x13, 13);
    assert_eq!(cpu.x14, 14);
    assert_eq!(cpu.x15, 15);
    assert_eq!(cpu.x16, 16);
    assert_eq!(cpu.x17, 17);
    assert_eq!(cpu.x18, 18);
    assert_eq!(cpu.x19, 19);
    assert_eq!(cpu.x20, 20);
    assert_eq!(cpu.x21, 21);
    assert_eq!(cpu.x22, 22);
    assert_eq!(cpu.x23, 23);
    assert_eq!(cpu.x24, 24);
    assert_eq!(cpu.x25, 25);
    assert_eq!(cpu.x26, 26);
    assert_eq!(cpu.x27, 27);
    assert_eq!(cpu.x28, 28);
    assert_eq!(cpu.x29, 29);
    assert_eq!(cpu.x30, 30);
    assert_eq!(cpu.x31, 31);

    //-- Contador de programa
    assert_eq!(cpu.pc, 0x7C);

    //-- ciclos
    assert_eq!(cpu.cycle, 31);

}

#[test]
fn test_addi1()
//────────────────────────────────────────────────
//  INSTRUCCION ADDI: TEST 2
//────────────────────────────────────────────────
{
    let code: Vec<u32> = vec![

        0x00200193,  // li x3, 2
        0x00000693,  // li x13, 0x00000000
        0x00068713,  // addi x14, x13, 0x00
        0x00000393,  // li x7, 0x00000000
        0x00771663,  // bne x14, x7, fail;

        //-- Pass:
        0x00100093,  // li x1, 1
        0x0000006F,  // j .

        //-- Fail:
        0x00000093,  //li x1, 0
        0x0000006F,  //j .
    ];

    //-- Crear la memoria con el programa
    let mem = Memory::from_u32(code);

    //-- Crear CPU
    let mut cpu = Cpurv::new(mem);

    //-- Ejecutar el programa!
    cpu.run(10);

    assert_eq!(cpu.x1, 1);
}

#[test]
fn test_addi2()
{
    let code: Vec<u32> = vec![
        //-- li x3, 2
        InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: 2 }.to_mcode(),

        //-- li x13, 0x00000000
        InstructionRV::Addi { rd: Reg::X13, rs1: Reg::X0, imm: 0x0000_0000 }
            .to_mcode(),

        //-- addi x14, x13, 0x00
        InstructionRV::Addi { rd: Reg::X14, rs1: Reg::X13, imm: 0x000 }
            .to_mcode(),

        //-- li x7, 0x00000000
        InstructionRV::Addi { rd: Reg::X7, rs1: Reg::X0, imm: 0x0000_0000 }
            .to_mcode(),

        //-- bne x14, x7, fail;
        InstructionRV::Bne { rs1: Reg::X14, rs2: Reg::X7, offs: 0x0C }
            .to_mcode(),
    
        //-- pass:
        //-- li x1, 1
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }
            .to_mcode(),
        //-- j .
        InstructionRV::Jal {rd: Reg::X0, offs: 0}.to_mcode(),

        //-- fail:
        //-- li x1, 0
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 0 }.to_mcode(), 
        //-- j .
        InstructionRV::Jal {rd: Reg::X0, offs: 0}.to_mcode(),
    ];

    //-- Crear la memoria con el programa
    let mem = Memory::from_u32(code);

    //-- Crear CPU
    let mut cpu = Cpurv::new(mem);

    //-- Ejecutar el programa!
    cpu.run(10);

}


