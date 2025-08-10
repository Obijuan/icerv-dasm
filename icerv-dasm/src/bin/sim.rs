//────────────────────────────────────────────────
//  SIMULADOR DE RV32I
//────────────────────────────────────────────────

use icerv_dasm::instructionrv::InstructionRV;
use icerv_dasm::regs::Reg;

//-- Modelado de la CPU RiscV
struct Cpurv {
    pc: u32,     //-- Contador de programa
    cycle: u32,  //-- Contador de ciclos (ciclo actual)

    //-- Registros RV (Salvo x0 que vale 0 siempre)
    x1: u32,    //-- ra
    x2: u32,    //-- sp
    x3: u32,    //-- gp
    x4: u32,    //-- tp
    x5: u32,    //-- t0
    x6: u32,    //-- t1
    x7: u32,    //-- t2
    x8: u32,    //-- s0
    x9: u32,    //-- s1
    x10: u32,   //-- a0
    x11: u32,   //-- a1
    x12: u32,   //-- a2
    x13: u32,   //-- a3
    x14: u32,   //-- a4
    x15: u32,   //-- a5
    x16: u32,   //-- a6
    x17: u32,   //-- a7
    x18: u32,   //-- s2
    x19: u32,   //-- s3
    x20: u32,   //-- s4
    x21: u32,   //-- s5
    x22: u32,   //-- s6
    x23: u32,   //-- s7
    x24: u32,   //-- s8
    x25: u32,   //-- s9
    x26: u32,   //-- s10
    x27: u32,   //-- s11
    x28: u32,   //-- t3
    x29: u32,   //-- t4
    x30: u32,   //-- t5
    x31: u32,   //-- t6
}

impl Cpurv {

    //────────────────────────────────────────────────
    //  Constructor de CPU
    //  Todos los registros se ponen a 0
    //────────────────────────────────────────────────    
    fn new() -> Self {
        Cpurv {
            pc: 0,
            cycle: 0,

            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
            x31: 0,
        }
    }

    //────────────────────────────────────────────────
    //  Mostrars todos los registros
    //────────────────────────────────────────────────    
    fn show(&self) {
        println!("");
        
        println!("🟢 x1: {:#010X}", self.x1);
        println!("🟢 x2: {:#010X}", self.x2);
        println!("🟢 x3: {:#010X}", self.x3);
        println!("🟢 x4: {:#010X}", self.x4);
        println!("🟢 x5: {:#010X}", self.x5);
        println!("🟢 x6: {:#010X}", self.x6);
        println!("🟢 x7: {:#010X}", self.x7);
        println!("🟢 x8: {:#010X}", self.x8);
        println!("🟢 x9: {:#010X}", self.x9);
        println!("🟢 x10: {:#010X}", self.x10);
        println!("🟢 x11: {:#010X}", self.x11);
        println!("🟢 x12: {:#010X}", self.x12);
        println!("🟢 x13: {:#010X}", self.x13);
        println!("🟢 x14: {:#010X}", self.x14);
        println!("🟢 x15: {:#010X}", self.x15);
        println!("🟢 x16: {:#010X}", self.x16);
        println!("🟢 x17: {:#010X}", self.x17);
        println!("🟢 x18: {:#010X}", self.x18);
        println!("🟢 x19: {:#010X}", self.x19);
        println!("🟢 x20: {:#010X}", self.x20);
        println!("🟢 x21: {:#010X}", self.x21);
        println!("🟢 x22: {:#010X}", self.x22);
        println!("🟢 x23: {:#010X}", self.x23);
        println!("🟢 x24: {:#010X}", self.x24);
        println!("🟢 x25: {:#010X}", self.x25);
        println!("🟢 x26: {:#010X}", self.x26);
        println!("🟢 x27: {:#010X}", self.x27);
        println!("🟢 x28: {:#010X}", self.x28);
        println!("🟢 x29: {:#010X}", self.x29);
        println!("🟢 x30: {:#010X}", self.x30);
        println!("🟢 x31: {:#010X}", self.x31);
        println!("➡️  pc: {:#010X}", self.pc);
        println!("⏱️  Ciclo: {}", self.cycle);
        println!("");
    }

    //────────────────────────────────────────────────
    //  Leer el registro indicado (Salvo PC)
    //──────────────────────────────────────────────── 
    fn read_reg(&self, reg: Reg) -> u32 {

        match reg {
            Reg::X0 => { 0 }
            Reg::X1 => { self.x1 }
            Reg::X2 => { self.x2 }
            Reg::X3 => { self.x3 }
            Reg::X4 => { self.x4 }
            Reg::X5 => { self.x5 }
            Reg::X6 => { self.x6 }
            Reg::X7 => { self.x7 }
            Reg::X8 => { self.x8 }
            Reg::X9 => { self.x9 }
            Reg::X10 => { self.x10 }
            Reg::X11 => { self.x11 }
            Reg::X12 => { self.x12 }
            Reg::X13 => { self.x13 }
            Reg::X14 => { self.x14 }
            Reg::X15 => { self.x15 }
            Reg::X16 => { self.x16 }
            Reg::X17 => { self.x17 }
            Reg::X18 => { self.x18 }
            Reg::X19 => { self.x19 }
            Reg::X20 => { self.x20 }
            Reg::X21 => { self.x21 }
            Reg::X22 => { self.x22 }
            Reg::X23 => { self.x23 }
            Reg::X24 => { self.x24 }
            Reg::X25 => { self.x25 }
            Reg::X26 => { self.x26 }
            Reg::X27 => { self.x27 }
            Reg::X28 => { self.x28 }
            Reg::X29 => { self.x29 }
            Reg::X30 => { self.x30 }
            Reg::X31 => { self.x31 }
        }
    }
    //────────────────────────────────────────────────
    //  Escribir un valor en el registro indicado
    // (Salvo pc)
    //──────────────────────────────────────────────── 
    fn write_reg(&mut self, reg: Reg, value: u32) {
        match reg {
            Reg::X0 => { }
            Reg::X1 => { self.x1 = value; }
            Reg::X2 => { self.x2 = value; }
            Reg::X3 => { self.x3 = value; }
            Reg::X4 => { self.x4 = value; }
            Reg::X5 => { self.x5 = value; }
            Reg::X6 => { self.x6 = value; }
            Reg::X7 => { self.x7 = value; }
            Reg::X8 => { self.x8 = value; }
            Reg::X9 => { self.x9 = value; }
            Reg::X10 => { self.x10 = value; }
            Reg::X11 => { self.x11 = value; }
            Reg::X12 => { self.x12 = value; }
            Reg::X13 => { self.x13 = value; }
            Reg::X14 => { self.x14 = value; }
            Reg::X15 => { self.x15 = value; }
            Reg::X16 => { self.x16 = value; }
            Reg::X17 => { self.x17 = value; }
            Reg::X18 => { self.x18 = value; }
            Reg::X19 => { self.x19 = value; }
            Reg::X20 => { self.x20 = value; }
            Reg::X21 => { self.x21 = value; }
            Reg::X22 => { self.x22 = value; }
            Reg::X23 => { self.x23 = value; }
            Reg::X24 => { self.x24 = value; }
            Reg::X25 => { self.x25 = value; }
            Reg::X26 => { self.x26 = value; }
            Reg::X27 => { self.x27 = value; }
            Reg::X28 => { self.x28 = value; }
            Reg::X29 => { self.x29 = value; }
            Reg::X30 => { self.x30 = value; }
            Reg::X31 => { self.x31 = value; }
        }
    }

    //────────────────────────────────────────────────
    //  Ejecutar una instrucción
    //──────────────────────────────────────────────── 
    fn exec(&mut self, inst: &InstructionRV) {
    
        println!("  ⚙️  {}", inst.to_string());

        //-- Ejecutar instruccion
        match inst {
            InstructionRV::Addi {rd, rs1, imm} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular resultado
                let res = ((rs1 as i32) + imm) as u32;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;

            }
            InstructionRV::Bne { rs1, rs2, offs } => {
                //-- Leer registro rs1
                let rs1 = self.read_reg(*rs1);

                //-- Leer registro rs2
                let rs2 = self.read_reg(*rs2);

                //-- Comprobar la condición
                if rs1 != rs2 {
                    //-- Ejecutar el salto
                    self.pc = (self.pc as i32 + *offs) as u32;
                }
                else {
                    //-- No se cumple condición. El PC apunta a la siguiente
                    //-- instruccion
                    self.pc += 4;
                }
            }
            _ => {}
        }

        //-- Todas las instrucciones consumen 1 ciclo
        self.cycle += 1;


    }



}

//────────────────────────────────────────────────
//  Ejecutar una instrucción
//──────────────────────────────────────────────── 

fn main()
{
    //-- Programa de test a ejecutar
    let prog1 = [
        //-- li x3, 2
        InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: 2 }, 
        //-- li x13, 0x00000000
        InstructionRV::Addi { rd: Reg::X13, rs1: Reg::X0, imm: 0x0000_0000 }, 
        //-- addi x14, x13, 0x00
        InstructionRV::Addi { rd: Reg::X14, rs1: Reg::X13, imm: 0x000 },
        //-- li x7, 0x00000000
        InstructionRV::Addi { rd: Reg::X7, rs1: Reg::X0, imm: 0x0000_0000 },
        //-- bne x14, x7, fail;
        InstructionRV::Bne { rs1: Reg::X14, rs2: Reg::X7, offs: 0x0C },

        //-- pass:
        //-- li x1, 1
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }, 
    ];

    let _prog2 = [
        InstructionRV::Bne { rs1: Reg::X14, rs2: Reg::X7, offs: 0x0 }
    ];

    //-- Crear CPU
    let mut cpu = Cpurv::new();

    //-- Mostrar estado inicial de la cpu
    cpu.show();

    //-- Seleccionar programa a ejecutar
    let prog = prog1;

    //-- Contador maximo de ciclos
    //-- (para evitar bucles infinitos)
    const MAX_CYCLES: u32 = 10;

    loop {
        //-- Obtener la direccion actual (de palabra)
        let addr = (cpu.pc >> 2) as usize;

        //-- Comprobar si la direccion está dentro del rango
        if addr >= prog.len() {
            break;
        }

        //-- Ejecutar instruccion
        cpu.exec(&prog[addr]);

        //-- Mostrar el estdo actual
        cpu.show();

        //-- Terminar cuando han transcurrido el máximo de ciclos
        if cpu.cycle >= MAX_CYCLES {
            break;
        }
    }

    if cpu.cycle == MAX_CYCLES {
        println!("CONTADOR DE CICLOS MAXIMO ALCANZADO!!");
    } else {
        println!("PROGRAMA TERMINADO")
    }

    // for inst in prog2 {
    //     //-- Ejecutar instruccion
    //     cpu.exec(&inst);

    //     //-- Mostrar estado actual
    //     cpu.show();
    // }
    println!();
}


#[test]
fn test_addi_1()
{
    //-- Programa de test a ejecutar
    let prog = [
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }, //-- addi x1, x0, 1
        InstructionRV::Addi { rd: Reg::X2, rs1: Reg::X0, imm: 2 }, //-- addi x2, x0, 2
        InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: 3 }, //-- addi x3, x0, 3
        InstructionRV::Addi { rd: Reg::X4, rs1: Reg::X0, imm: 4 }, //-- addi x4, x0, 4
        InstructionRV::Addi { rd: Reg::X5, rs1: Reg::X0, imm: 5 }, //-- addi x5, x0, 5
        InstructionRV::Addi { rd: Reg::X6, rs1: Reg::X0, imm: 6 }, //-- addi x6, x0, 6
        InstructionRV::Addi { rd: Reg::X7, rs1: Reg::X0, imm: 7 }, //-- addi x7, x0, 7
        InstructionRV::Addi { rd: Reg::X8, rs1: Reg::X0, imm: 8 }, //-- addi x8, x0, 8
        InstructionRV::Addi { rd: Reg::X9, rs1: Reg::X0, imm: 9 }, //-- addi x9, x0, 9
        InstructionRV::Addi { rd: Reg::X10, rs1: Reg::X0, imm: 10 }, //-- addi x10, x0, 10
        InstructionRV::Addi { rd: Reg::X11, rs1: Reg::X0, imm: 11 }, //-- addi x11, x0, 11
        InstructionRV::Addi { rd: Reg::X12, rs1: Reg::X0, imm: 12 }, //-- addi x12, x0, 12
        InstructionRV::Addi { rd: Reg::X13, rs1: Reg::X0, imm: 13 }, //-- addi x13, x0, 13
        InstructionRV::Addi { rd: Reg::X14, rs1: Reg::X0, imm: 14 }, //-- addi x14, x0, 14
        InstructionRV::Addi { rd: Reg::X15, rs1: Reg::X0, imm: 15 }, //-- addi x15, x0, 15
        InstructionRV::Addi { rd: Reg::X16, rs1: Reg::X0, imm: 16 }, //-- addi x16, x0, 16
        InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X0, imm: 17 }, //-- addi x17, x0, 17
        InstructionRV::Addi { rd: Reg::X18, rs1: Reg::X0, imm: 18 }, //-- addi x18, x0, 18
        InstructionRV::Addi { rd: Reg::X19, rs1: Reg::X0, imm: 19 }, //-- addi x19, x0, 19
        InstructionRV::Addi { rd: Reg::X20, rs1: Reg::X0, imm: 20 }, //-- addi x20, x0, 20
        InstructionRV::Addi { rd: Reg::X21, rs1: Reg::X0, imm: 21 }, //-- addi x21, x0, 21
        InstructionRV::Addi { rd: Reg::X22, rs1: Reg::X0, imm: 22 }, //-- addi x22, x0, 22
        InstructionRV::Addi { rd: Reg::X23, rs1: Reg::X0, imm: 23 }, //-- addi x23, x0, 23
        InstructionRV::Addi { rd: Reg::X24, rs1: Reg::X0, imm: 24 }, //-- addi x24, x0, 24
        InstructionRV::Addi { rd: Reg::X25, rs1: Reg::X0, imm: 25 }, //-- addi x25, x0, 25
        InstructionRV::Addi { rd: Reg::X26, rs1: Reg::X0, imm: 26 }, //-- addi x26, x0, 26
        InstructionRV::Addi { rd: Reg::X27, rs1: Reg::X0, imm: 27 }, //-- addi x27, x0, 27
        InstructionRV::Addi { rd: Reg::X28, rs1: Reg::X0, imm: 28 }, //-- addi x28, x0, 28
        InstructionRV::Addi { rd: Reg::X29, rs1: Reg::X0, imm: 29 }, //-- addi x29, x0, 29
        InstructionRV::Addi { rd: Reg::X30, rs1: Reg::X0, imm: 30 }, //-- addi x30, x0, 30
        InstructionRV::Addi { rd: Reg::X31, rs1: Reg::X0, imm: 31 }, //-- addi x31, x0, 31
    ];

    //-- Crear CPU
    let mut cpu = Cpurv::new();

    for inst in prog {

        //-- Ejecutar instruccion
        cpu.exec(&inst);
    }
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
