//────────────────────────────────────────────────
//  CPU RISCV
//────────────────────────────────────────────────
use crate::instructionrv::InstructionRV; 
use crate::memory::Memory;
use crate::ansi;
use crate::regs::Reg;


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
pub struct Cpurv {
    pub pc: u32,         //-- Contador de programa
    pub cycle: u32,      //-- Contador de ciclos (ciclo actual)
    max_cycles: u32, //-- Numero maximo de ciclos a ejecutar
    state: CpuState, //-- Estado de la CPU
    mem: Memory,     //-- Memoria conectada a la cpu

    //-- Registros RV (Salvo x0 que vale 0 siempre)
    pub x1: u32,    //-- ra
    pub x2: u32,    //-- sp
    pub x3: u32,    //-- gp
    pub x4: u32,    //-- tp
    pub x5: u32,    //-- t0
    pub x6: u32,    //-- t1
    pub x7: u32,    //-- t2
    pub x8: u32,    //-- s0
    pub x9: u32,    //-- s1
    pub x10: u32,   //-- a0
    pub x11: u32,   //-- a1
    pub x12: u32,   //-- a2
    pub x13: u32,   //-- a3
    pub x14: u32,   //-- a4
    pub x15: u32,   //-- a5
    pub x16: u32,   //-- a6
    pub x17: u32,   //-- a7
    pub x18: u32,   //-- s2
    pub x19: u32,   //-- s3
    pub x20: u32,   //-- s4
    pub x21: u32,   //-- s5
    pub x22: u32,   //-- s6
    pub x23: u32,   //-- s7
    pub x24: u32,   //-- s8
    pub x25: u32,   //-- s9
    pub x26: u32,   //-- s10
    pub x27: u32,   //-- s11
    pub x28: u32,   //-- t3
    pub x29: u32,   //-- t4
    pub x30: u32,   //-- t5
    pub x31: u32,   //-- t6
}

impl Cpurv {

    //────────────────────────────────────────────────
    //  Constructor de CPU
    //  Se crea una CPU inicializada y asociada a la memoria dada
    //  La CPU se lleva al estado de reset
    //  Todos los registros x se ponen a 0
    //────────────────────────────────────────────────    
    pub fn new(mem: Memory) -> Self {

        //-- Por defecto se usa una memoria nula
        //let data:Vec<u8> = vec![0; 0];

        Cpurv {
            pc: 0,
            cycle: 0,
            max_cycles: 0xFFFF_FFFF,
            state: CpuState::RESET,
            mem: mem,

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
    pub fn show(&self) {
        println!("┌─────────────────────────────────────────────────────────────────────┐");

        print!("│  {}x0:{} {:#010X}   {}x1:{} {:#010X}", ansi::BLUE, ansi::RESET, 0, 
                ansi::BLUE, ansi::RESET, self.x1);
        print!("   {}x2:{} {:#010X}   {}x3:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x2, ansi::BLUE, ansi::RESET, self.x3);

        print!("│  {}x4:{} {:#010X}   {}x5:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x4, ansi::BLUE, ansi::RESET, self.x5);
        print!("   {}x6:{} {:#010X}   {}x7:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x6, ansi::BLUE, ansi::RESET, self.x7);

        print!("│  {}x8:{} {:#010X}   {}x9:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x8, ansi::BLUE, ansi::RESET, self.x9);
        print!("  {}x10{}: {:#010X}  {}x11:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET, 
                self.x10, ansi::BLUE, ansi::RESET, self.x11);

        print!("│ {}x12:{} {:#010X}  {}x13:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x12, ansi::BLUE, ansi::RESET, self.x13);
        print!("  {}x14:{} {:#010X}  {}x15:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x14, ansi::BLUE, ansi::RESET, self.x15);

        print!("│ {}x16:{} {:#010X}  {}x17:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x16, ansi::BLUE, ansi::RESET, self.x17);
        print!("  {}x18:{} {:#010X}  {}x19:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x18, ansi::BLUE, ansi::RESET, self.x19);

        print!("│ {}x20:{} {:#010X}  {}x21:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x20, ansi::BLUE, ansi::RESET, self.x21);
        print!("  {}x22:{} {:#010X}  {}x23:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x22, ansi::BLUE, ansi::RESET, self.x23);

        print!("│ {}x24:{} {:#010X}  {}x25:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x24, ansi::BLUE, ansi::RESET, self.x25);
        print!("  {}x26:{} {:#010X}  {}x27:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x26, ansi::BLUE, ansi::RESET, self.x27);

        print!("│ {}x28:{} {:#010X}  {}x29:{} {:#010X}", ansi::BLUE, ansi::RESET, 
                self.x28, ansi::BLUE, ansi::RESET, self.x29);
        print!("  {}x22:{} {:#010X}  {}x23:{} {:#010X}  │\n", ansi::BLUE, ansi::RESET,
                self.x30, ansi::BLUE, ansi::RESET, self.x31);
        
        println!("├───────────────────┬──────────────────────┬──────────────────────────┤");

        print!("│ ➡️  pc: {:#010X} │", self.pc);

        print!(" ⏱️  Ciclo: {:4}", self.cycle);
        if self.cycle < self.max_cycles {
            print!("      ");
        } else {
            print!(" (MAX)");
        }
        print!(" │");

        print!(" 🚨  Estado: {:5}        │\n", self.state.to_str());
        println!("└───────────────────┴──────────────────────┴──────────────────────────┘");

    }

    //────────────────────────────────────────────────
    //  Leer el registro indicado (Salvo PC)
    //──────────────────────────────────────────────── 
    pub fn read_reg(&self, reg: Reg) -> u32 {

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
    pub fn write_reg(&mut self, reg: Reg, value: u32) {
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
    pub fn exec(&mut self, inst: &InstructionRV) {
    
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
            //──────────────────────────────────
            //  Instrucciones aritméticas tipo I
            //──────────────────────────────────
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
            InstructionRV::Slli {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Restringir el valor del desplazamiento a 5 bits
                let desp: u32 = (*imm as u32) & 0x1F;

                //-- Calcular el resultado
                let res: u32 = rs1 << desp;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Slti {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular el resultado
                let res: bool = (rs1 as i32) < *imm;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Sltiu {rd, rs1, imm} => {
                 //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular el resultado
                let res: bool = (rs1 as u32) < (*imm as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Xori {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular el resultado
                let res: u32 = (rs1 as u32) ^ (*imm as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Srli {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Restringir el valor del desplazamiento a 5 bits
                let desp: u32 = (*imm as u32) & 0x1F;

                //-- Calcular el resultado
                let res: u32 = rs1 >> desp;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Srai {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Restringir el valor del desplazamiento a 5 bits
                let desp: u32 = (*imm as u32) & 0x1F;

                //-- Calcular el resultado
                let res = (rs1 as i32) >> desp;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;   
            }
            InstructionRV::Ori {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular el resultado
                let res: u32 = (rs1 as u32) | (*imm as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Andi {rd, rs1, imm} => {
                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular el resultado
                let res: u32 = (rs1 as u32) & (*imm as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            //──────────────────────────────────
            //  Instrucciones tipo I: LOAD
            //──────────────────────────────────
            InstructionRV::Lb {rd, offs, rs1} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Leer el byte de memoria como byte con signo
                let res:i8 = self.mem.read8(addr) as i8; 

                //-- Guardar resultado en el registro destino
                //-- Al pasar a u32 se extiende el signo
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Lh {rd, offs, rs1} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Leer la media palabra de memoria, con signo
                let res:i16 = self.mem.read16(addr) as i16; 

                //-- Guardar resultado en el registro destino
                //-- Al pasar a u32 se extiende el signo
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Lw {rd, offs, rs1} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Leer la media palabra de memoria, con signo
                let res:i32 = self.mem.read32(addr) as i32; 

                //-- Guardar resultado en el registro destino
                //-- Al pasar a u32 se extiende el signo
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Lbu { rd, offs, rs1} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Leer la media palabra de memoria, con signo
                let res:u32 = self.mem.read8(addr) as u32; 

                //-- Guardar resultado en el registro destino
                //-- Al pasar a u32 se extiende el signo
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Lhu { rd, offs, rs1} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Leer la media palabra de memoria, con signo
                let res:u16 = self.mem.read16(addr) as u16; 

                //-- Guardar resultado en el registro destino
                //-- Al pasar a u32 se extiende el signo
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            //──────────────────────────────────
            //  Instrucciones tipo R
            //──────────────────────────────────
            InstructionRV::Add {rd, rs1, rs2} => {
                
                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular resultado
                //-- Se calcula para 64 bits
                let t_rs1 = rs1 as i64;
                let t_rs2: i64 = rs2 as i64;
                let t_res: i64 = t_rs1 + t_rs2;

                //-- Truncar resultado a 32 bits
                let res:u32 = (t_res & 0xFFFF_FFFF) as u32;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;

            }
            InstructionRV::Sub {rd, rs1, rs2} => {
                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular resultado
                //-- Se calcula para 64 bits
                let t_rs1 = rs1 as i64;
                let t_rs2: i64 = rs2 as i64;
                let t_res: i64 = t_rs1 - t_rs2;

                //-- Truncar resultado a 32 bits
                let res:u32 = (t_res & 0xFFFF_FFFF) as u32;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Sll {rd, rs1, rs2} => {

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Truncar el valor del desplazamiento a 5 bits
                let rs2 = rs2 & 0x1F;

                //-- Calcular resultado
                let res = (rs1 as u32) << (rs2 as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Slt {rd, rs1, rs2} => {

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular el resultado
                let res: bool = (rs1 as i32) < (rs2 as i32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Sltu {rd, rs1, rs2} => {
            
                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular el resultado
                let res: bool = rs1 < rs2;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Xor {rd, rs1, rs2} => {

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular el resultado
                let res: u32 = (rs1 as u32) ^ (rs2 as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Srl {rd, rs1, rs2} => {
                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Truncar el valor del desplazamiento a 5 bits
                let rs2 = rs2 & 0x1F;

                //-- Calcular resultado
                let res = (rs1 as u32) >> (rs2 as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Or {rd, rs1, rs2} => {

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular el resultado
                let res: u32 = (rs1 as u32) | (rs2 as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::And {rd, rs1, rs2} => {
                
                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Calcular el resultado
                let res: u32 = (rs1 as u32) & (rs2 as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Sra {rd, rs1, rs2} => {
                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Truncar el valor del desplazamiento a 5 bits
                let rs2 = rs2 & 0x1F;

                //-- Calcular resultado
                let res = (rs1 as i32) >> (rs2 as u32);

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            //──────────────────────────────────
            //  Instrucciones tipo S
            //──────────────────────────────────
            InstructionRV::Sb { rs2, offs, rs1 } => {

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Escribir el byte en memoria
                self.mem.write8(addr, rs2 as u8);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Sh { rs2, offs, rs1 } => {

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Escribir el byte en memoria
                self.mem.write16(addr, rs2 as u16);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            InstructionRV::Sw { rs2, offs, rs1 } => {

                //-- Leer valor del registro fuente 2
                let rs2 = self.read_reg(*rs2);

                //-- Leer valor del registro fuente 1
                let rs1 = self.read_reg(*rs1);

                //-- Calcular la direccion de memoria
                let addr:u32 = ((rs1 as i32) + offs) as u32;

                //-- Escribir el byte en memoria
                self.mem.write32(addr, rs2 as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            }
            //──────────────────────────────────
            //  Instrucciones tipo B
            //──────────────────────────────────
            InstructionRV::Beq { rs1, rs2, offs } => {
                //-- Leer registro rs1
                let rs1 = self.read_reg(*rs1);

                //-- Leer registro rs2
                let rs2 = self.read_reg(*rs2);

                //-- Comprobar la condición
                if rs1 == rs2 {
                    //-- Ejecutar el salto
                    self.pc = (self.pc as i32 + *offs) as u32;
                }
                else {
                    //-- No se cumple condición. El PC apunta a la siguiente
                    //-- instruccion
                    self.pc += 4;
                }
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
            InstructionRV::Blt { rs1, rs2, offs } => {
                //-- Leer registro rs1
                let rs1 = self.read_reg(*rs1) as i32;

                //-- Leer registro rs2
                let rs2 = self.read_reg(*rs2) as i32;

                //-- Comprobar la condición
                if rs1 < rs2 {
                    //-- Ejecutar el salto
                    self.pc = (self.pc as i32 + *offs) as u32;
                }
                else {
                    //-- No se cumple condición. El PC apunta a la siguiente
                    //-- instruccion
                    self.pc += 4;
                }
            }
            InstructionRV::Bge { rs1, rs2, offs } => {
                //-- Leer registro rs1
                let rs1 = self.read_reg(*rs1) as i32;

                //-- Leer registro rs2
                let rs2 = self.read_reg(*rs2) as i32;

                //-- Comprobar la condición
                if rs1 >= rs2 {
                    //-- Ejecutar el salto
                    self.pc = (self.pc as i32 + *offs) as u32;
                }
                else {
                    //-- No se cumple condición. El PC apunta a la siguiente
                    //-- instruccion
                    self.pc += 4;
                }
            }
            InstructionRV::Bltu { rs1, rs2, offs } => {
                //-- Leer registro rs1
                let rs1 = self.read_reg(*rs1);

                //-- Leer registro rs2
                let rs2 = self.read_reg(*rs2);

                //-- Comprobar la condición
                if rs1 < rs2 {
                    //-- Ejecutar el salto
                    self.pc = (self.pc as i32 + *offs) as u32;
                }
                else {
                    //-- No se cumple condición. El PC apunta a la siguiente
                    //-- instruccion
                    self.pc += 4;
                }
            }
            InstructionRV::Bgeu { rs1, rs2, offs } => {
                //-- Leer registro rs1
                let rs1 = self.read_reg(*rs1);

                //-- Leer registro rs2
                let rs2 = self.read_reg(*rs2);

                //-- Comprobar la condición
                if rs1 >= rs2 {
                    //-- Ejecutar el salto
                    self.pc = (self.pc as i32 + *offs) as u32;
                }
                else {
                    //-- No se cumple condición. El PC apunta a la siguiente
                    //-- instruccion
                    self.pc += 4;
                }
            }
            //──────────────────────────────────
            //  Instrucciones tipo U: LUI
            //──────────────────────────────────
            InstructionRV::Lui {rd, imm} => {

                //-- Calcular resultado
                let res = imm << 12;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, res as u32);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;

            }
            InstructionRV::Auipc {rd, imm} => {

                //-- Calcular resultado: Direccion destino
                let addr: u32 = ((self.pc as i32) + (imm<<12)) as u32;

                //-- Escribir resultado en registro destino
                self.write_reg(*rd, addr);

                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;
            } 
            //──────────────────────────────────
            //  Instrucciones tipo J
            //──────────────────────────────────
            //-- 🚧 TODO 🚧
            InstructionRV::Jal {rd, offs} => {

                //-- Guardar direccion de retorno
                self.write_reg(*rd, self.pc+4);

                //-- Actualizar el pc
                self.pc = (self.pc as i32 + *offs) as u32;
            }
            InstructionRV::Jalr {rd, offs, rs1} => {

                //-- Leer valor del registro fuente
                let rs1 = self.read_reg(*rs1);

                 //-- Guardar direccion de retorno
                self.write_reg(*rd, self.pc+4);

                //-- Actualizar el pc
                self.pc = (*offs + (rs1 as i32)) as u32;

               
            }
            //──────────────────────────────────
            //  Instrucciones tipo ecall
            //──────────────────────────────────
            InstructionRV::Ecall => {
                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;

                //-- DEBUG!
                //println!("ECALL!!");
            }
            InstructionRV::Ebreak => {
                //-- Incrementar pc para apuntar a la siguiente instruccion
                self.pc += 4;

                //-- DEBUG!
                //println!("EBREAK!!");
            }
            //──────────────────────────────────
            //  Instrucciones DESCONOCIDA
            //  (o NO IMPLEMENTADA)
            //──────────────────────────────────
            InstructionRV::Unknown => {
                println!("INSTRUCCION DESCONOCIDA!");
                self.state = CpuState::HALT;
                return
            }
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

    //────────────────────────────────────────────────
    //  Realizar un paso de simulacion
    //  * Leer instrucción actual
    //  * Ejecutar instrucción
    //──────────────────────────────────────────────── 
    pub fn step(&mut self) {
        //-- Obtener la direccion actual
        let addr = self.pc;

        //-- Comprobar si la direccion está dentro del rango
        if addr >= self.mem.size() as u32 {
            self.state = CpuState::HALT;
            return;
        }

        //-- Leer instruccion de memoria
        let mcode = self.mem.read32(addr);

        //-- Convertir codigo maquina a tipo instruccion
        let inst = InstructionRV::from_mcode(mcode as u32);

        //-- Ejecutar instruccion!
        self.exec(&inst);
    }
    //────────────────────────────────────────────────
    //  Ejecutar el programa que está en memoria
    //──────────────────────────────────────────────── 
    pub fn run(&mut self, max_cycles: u32) {

        //-- Configurar los ciclos máximos
        self.max_cycles = max_cycles;

        //-- Bucle principal de Ejecucion!!
        loop {

            //-- Realizar un paso de simulacion
            self.step();

            //-- Mostrar el estado actual
            //self.show();

            //-- Terminar cuando han transcurrido el máximo de ciclos
            if self.state == CpuState::HALT {
                break;
            }
        }
    }


}


