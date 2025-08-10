//────────────────────────────────────────────────
//  SIMULADOR DE RV32I
//────────────────────────────────────────────────

use icerv_dasm::instructionrv::InstructionRV;
use icerv_dasm::regs::Reg;

//-- Modelado de la CPU RiscV
struct Cpurv {
    pc: u32,    //-- Contador de programa

    //-- Registros RV (Salvo x0 que vale 0 siempre)
    x1: u32,    //-- ra
}

impl Cpurv {

    //────────────────────────────────────────────────
    //  Constructor de CPU
    //  Todos los registros se ponen a 0
    //────────────────────────────────────────────────    
    fn new() -> Self {
        Cpurv {
            pc: 0,
            x1: 0,
        }
    }

    //────────────────────────────────────────────────
    //  Mostrars todos los registros
    //────────────────────────────────────────────────    
    fn show(&self) {
        println!("");
        println!("🟢 pc: {:#010X}", self.pc);
        println!("🟢 x1: {:#010X}", self.x1);
        println!();
    }

    //────────────────────────────────────────────────
    //  Leer el registro indicado (Salvo PC)
    //──────────────────────────────────────────────── 
    fn read_reg(&self, reg: Reg) -> u32 {

        match reg {
            Reg::X0 => { 0 }
            Reg::X1 => { self.x1 }
            _ => { 0xFFFF_FFFF }
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
            _ => {}
        }
    }

    //────────────────────────────────────────────────
    //  Ejecutar una instrucción
    //──────────────────────────────────────────────── 
    fn exec(&mut self, inst: &InstructionRV) {
    
        println!("  ➡️  {}", inst.to_string());

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
            _ => {}
        }

    }



}

//────────────────────────────────────────────────
//  Ejecutar una instrucción
//──────────────────────────────────────────────── 

fn main()
{
    //-- Programa de test a ejecutar
    let prog1 = [
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }, //-- addi x1, x0, 1
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X1, imm: 1 }, //-- addi x1, x1, 1
    ];

    //-- Crear CPU
    let mut cpu = Cpurv::new();

    //-- Mostrar estado inicial de la cpu
    cpu.show();

    for inst in prog1 {

        //-- Ejecutar instruccion
        cpu.exec(&inst);

        //-- Mostrar estado actual
        cpu.show();
    }
    println!();
}
