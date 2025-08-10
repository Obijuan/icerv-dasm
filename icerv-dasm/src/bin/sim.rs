//────────────────────────────────────────────────
//  SIMULADOR DE RV32I
//────────────────────────────────────────────────

use icerv_dasm::instructionrv::InstructionRV;
use icerv_dasm::regs::Reg;

//-- Modelado de los registros del riscv
struct Regs {
    x1: u32
}

impl Regs {

    //────────────────────────────────────────────────
    //  Constructor de Registros
    //  Todos los registros se ponen a 0
    //────────────────────────────────────────────────    
    fn new() -> Self {
        Regs {
            x1: 0,
        }
    }

    //────────────────────────────────────────────────
    //  Mostrars todos los registros
    //────────────────────────────────────────────────    
    fn show(&self) {
        println!("Registro x1: {:#010X}", self.x1);
    }

    //────────────────────────────────────────────────
    //  Leer el registro indicado
    //──────────────────────────────────────────────── 
    fn read(&self, reg: Reg) -> u32 {

        match reg {
            Reg::X0 => { 0 }
            Reg::X1 => { self.x1 }
            _ => { 0xFFFF_FFFF }
        }
    }
    //────────────────────────────────────────────────
    //  Escribir un valor en el registro indicado
    //──────────────────────────────────────────────── 
    fn write(&mut self, reg: Reg, value: u32) {
        match reg {
            Reg::X0 => { }
            Reg::X1 => { self.x1 = value; }
            _ => {}
        }
    }


}

//────────────────────────────────────────────────
//  Ejecutar una instrucción
//──────────────────────────────────────────────── 
fn exec(regs:&mut Regs, inst:&InstructionRV) 
{
    println!();
    println!("Exec: {}", inst.to_string());

    //-- Ejecutar instruccion
    match inst {
        InstructionRV::Addi {rd, rs1, imm} => {
            
            //-- Leer valor del registro fuente
            let rs1 = regs.read(*rs1);

            //-- Calcular resultado
            let res = ((rs1 as i32) + imm) as u32;

            //-- Escribir resultado en registro destino
            regs.write(*rd, res);

        }
        _ => {}
    }
    println!();
}

fn main()
{
    //-- Crear banco registros
    let mut regs = Regs::new();

    //-- Mostrar estado actual
    regs.show();

    //-- Definir instruccion a ejecutar
    let inst = InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 };

    //-- Ejecutar instruccion
    exec(&mut regs, &inst);

    //-- Mostrar estado actual    
    regs.show();


}
