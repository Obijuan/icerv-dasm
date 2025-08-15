//────────────────────────────────────────────────
//  SIMULADOR DE RV32I
//────────────────────────────────────────────────

use std::{fs::File, io::Read};

use icerv_dasm::instructionrv::InstructionRV;
use icerv_dasm::regs::Reg;
use icerv_dasm::ansi;

//-- Estado de la cpu
#[derive(PartialEq)]
enum CpuState {
    RESET,  //-- Cpu en estado inicial
    RUN,    //-- Cpu en modo normal, ejecutando instrucciones
    HALT,   //-- Cpu detenida, bien por llegar al maximo de ciclos
            //-- o bien por completar el programa
}

impl CpuState {
    fn to_str(&self) -> String {
        match self {
            CpuState::RESET => {
                format!("RESET")
            }
            CpuState::RUN => {
                format!("RUN")
            }
            CpuState::HALT => {
                format!("HALT")
            }
        }
    }
}

//-- Modelado de la CPU RiscV
struct Cpurv {
    pc: u32,         //-- Contador de programa
    cycle: u32,      //-- Contador de ciclos (ciclo actual)
    max_cycles: u32, //-- Numero maximo de ciclos a ejecutar
    state: CpuState, //-- Estado de la CPU

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
    //  La CPU se lleva al estado de reset
    //  Todos los registros x se ponen a 0
    //────────────────────────────────────────────────    
    fn new() -> Self {
        Cpurv {
            pc: 0,
            cycle: 0,
            max_cycles: 0xFFFF_FFFF,
            state: CpuState::RESET,

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

        if self.cycle < self.max_cycles {
            println!("⏱️  Ciclo: {}", self.cycle);
        } else {
            println!("⏱️  Ciclo: {} (MAX)", self.cycle);
        }
        
        println!("🚨  Estado: {}", self.state.to_str());
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
    
        //-- Si la cpu está detenida (HALT), no se ejecuta ya nada
        if self.state == CpuState::HALT {
            println!("Estado CPU: HALT");
            return
        }

        //-- Por defecto llevamos la cpu a RUN
        self.state = CpuState::RUN;

        println!("  ⚙️  {}", inst.to_string());

        //-- Ejecutar instruccion
        match inst {
            InstructionRV::Unknown => {
                println!("INSTRUCCION DESCONOCIDA!");
                self.state = CpuState::HALT;
                return
            }
            InstructionRV::Addi {rd, rs1, imm} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular resultado
                //-- Se calcula para 64 bits
                let t_rs1 = rs1 as i64;
                let t_imm: i64 = *imm as i64;
                let t_res: i64 = t_rs1 + t_imm;

                //-- Truncar resultado a 32 bits
                let res:u32 = (t_res & 0xFFFF_FFFF) as u32;

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
            InstructionRV::Jal {rd, offs} => {

                //-- Guardar direccion de retorno
                self.write_reg(*rd, self.pc+4);

                //-- Actualizar el pc
                self.pc = (self.pc as i32 + *offs) as u32;
            }
            InstructionRV::Lui {rd, imm} => {

                //-- Calcular resultado
                let res = imm << 12;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;

            },
            _ => {}
        }

        //-- Todas las instrucciones consumen 1 ciclo
        self.cycle += 1;

        //-- Evaluar los ciclos máximos
        if self.cycle >= self.max_cycles {

            //-- Parar la cpu
            self.state = CpuState::HALT;
        }


    }



}


//────────────────────────────────────────────────
//  Ejecutar un programa dado en código máquina
//──────────────────────────────────────────────── 
fn run_mcode(prog: &[u32], max_cycles: u32) -> Cpurv
{
    //-- Crear CPU
    let mut cpu = Cpurv::new();

    //-- Configurar los ciclos máximos
    cpu.max_cycles = max_cycles;

    //-- Mostrar estado inicial de la cpu
    cpu.show();

    loop {
        //-- Obtener la direccion actual (de palabra)
        let addr = (cpu.pc >> 2) as usize;

        //-- Comprobar si la direccion está dentro del rango
        if addr >= prog.len() {
            break;
        }

        //-- Obtener instruccion en codigo maquina
        let mcode = &prog[addr];

        //-- Convertir codigo maquina a tipo instruccion
        let inst = InstructionRV::from_mcode(*mcode as u32);

        //-- Ejecutar instruccion!
        cpu.exec(&inst);

        //-- Mostrar el estado actual
        //cpu.show();

        //-- Terminar cuando han transcurrido el máximo de ciclos
        if cpu.state == CpuState::HALT {
            break;
        }
    }

    println!("PROGRAMA TERMINADO");
    cpu
}


fn sim(fich: String)
{
    println!("{}{}{}",ansi::BLUE, fich, ansi::RESET);

    //-- Abrir fichero
    let ofile = File::open(fich);
    let mut file = match ofile {
        Ok(value) => {
            value
        }
        Err(error) => {
            println!("{}Error: {}{}", ansi::RED, error, ansi::RESET);
            println!();
            return
        }
    };

    //-- Crear un buffer para almacenar los 4 bytes de la instruccion
    let mut buffer = [0; 4];

    //-- Buffer donde colocar las instrucciones
    let mut buffer_insts: Vec<u32> = Vec::new();
    
    //-- Leer fichero de 4 en 4 bytes
    while file.read_exact(&mut buffer).is_ok() {

        // Convierte los 4 bytes a un entero de 32 bits sin signo (u32)
        let instr = u32::from_le_bytes(buffer);

        // Meter la instruccion en el buffer de instrucciones
        buffer_insts.push(instr);
    }

    println!("Tamaño: {} Instrucciones", buffer_insts.len());

    let cpu = run_mcode(&buffer_insts, 210);
    assert_eq!(cpu.x1, 1);
    cpu.show();
    
}


fn main() 
{

    //-- Borrar la pantalla
    print!("{}", ansi::CLS);

    //-- Leer primer argumento
    // let arg = std::env::args().nth(1);
    // let _fich = match arg {
    //     Some(value) => {
    //         value
    //     }
    //     None => {
    //         print!("{}", ansi::RED);
    //         println!("Error: Fichero ejecutable NO especificado");
    //         print!("{}", ansi::RESET);
    //         println!("  Uso: sim fichero");
    //         return;
    //     }
    // };

    //-- Leer programa de prueba desde un fichero
    let fich = String::from("asm/addi.bin");

    //-- Ejecutar programa
    sim(fich);

}

#[test]
fn test_addi() 
{
    let fich = String::from("asm/addi.bin");
    sim(fich);
}


#[test]
fn test_addi_1()
{
    //-- Programa de test a ejecutar
    let prog = [
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

    //-- Ejecutar el programa!
    let cpu = run_mcode(&prog, 100);

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
    let code = [

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

    let cpu = run_mcode(&code, 10);
    assert_eq!(cpu.x1, 1);
}

#[test]
fn test_addi2()
{
    let code = [
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

    run_mcode(&code, 10);
}


