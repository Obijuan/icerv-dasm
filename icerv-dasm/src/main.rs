//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────

//-- Instrucciones RV32I
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

#[cfg(test)]
mod tests;
mod regs;
mod instructionrv;
mod mcode;
mod opcoderv;

//-- Registros del RISCV
use regs::Reg;
use instructionrv::InstructionRV;
use mcode::MCode;
use opcoderv::OpcodeRV;

//────────────────────────────────────────────────
//  CONSTANTES PARA ACCESO A LA ISA DEL RISCV   
//────────────────────────────────────────────────
//  Definir anchura de campos. Estos campos son los que luego se llevan
//  a la posición concreta en la instrucción, para calcular la máscara
//  con la que extraer el campo
//───────────────────────────
//  ANCHURAS de LOS CAMPOS
//───────────────────────────
const FIELD_1B: u32 = 0x01;  //-- Campo de 1 bit
const FIELD_8B: u32 = 0xFF;  //-- Campo de 8 bits
const FIELD_10B: u32 = 0x3FF; //-- Campo de 10 bits
const FIELD_20B: u32 = 0xFFFFF; //-- Campo de 20 bits
//────────────────────────────────────────────────
//  POSICIONES de LOS CAMPOS
//────────────────────────────────────────────────
const IMM20_POS: u8 = 12; 
const OFFSET10_POS: u8 = 21;
const OFFSET8_POS: u8 = 12;
const OFFSET1_POS: u8 = 20;

//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
//  Se calculan desplazando los campos de la anchura correspondiente
//  a la posición del campo
//──────────────────────────────────────────────
const IMM20_MASK: u32 = FIELD_20B << IMM20_POS; 
const OFFSET8_MASK: u32 = FIELD_8B << OFFSET8_POS; 
const OFFSET10_MASK: u32 = FIELD_10B << OFFSET10_POS;
const OFFSET1_MASK: u32 = FIELD_1B << OFFSET1_POS;

fn get_imm20(inst: u32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Inmediato de 20 bits de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  let imm20: u32 = (inst & IMM20_MASK) >> IMM20_POS;

  //-- Convertir el valor a i32
  //-- Hay que extender el signo
  sign_ext20(imm20 as i32)
}


fn get_offset_jal(inst: u32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Valor del offset para instrucciones jal
// offset[20|10:1|11|19:12]
//────────────────────────────────────────────────
    //-- Extraer el bit de signo
    let sign: u32 = (inst & 0x8000_0000) >> 31;

    //-- Extraer los bits 19-12
    let offset8: u32 = (inst & OFFSET8_MASK) >> OFFSET8_POS;

    //-- Extraer el bit 11
    let b11: u32 = (inst & OFFSET1_MASK) >> OFFSET1_POS;

    //-- Extraer los bits 10-1
    let offset10: u32 = (inst & OFFSET10_MASK) >> OFFSET10_POS;

    //-- Construir el offset final a partir de todos sus componentes
    let offset: u32 = (sign << 20) | (offset8 << 12) | b11 << 11 |
                      (offset10 << 1); 
                      

    sign_ext21(offset as i32)
}

fn print_fields(inst: u32) {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Imprime los campos de la instrucción
//────────────────────────────────────────────────

    //-- Extraer los campos de la instrucción
    //let opcode = get_opcode(inst);

    let mcode = MCode::new(inst);
    let opcode = mcode.opcode();
    let rd: Reg = mcode.rd();
    let rs1: Reg = mcode.rs1();
    let func3: u32 = mcode.func3();
    let imm:i32 = mcode.imm12();

    let rs2 = mcode.rs2();
    let func7 = mcode.func7();

    //-- Imprimir los campos extraídos
    println!("   - Opcode: {:#4X}", opcode as u32);
    println!("   - rd: x{}", rd as u8);
    println!("   - func3: {:#05b}", func3);
    println!("   - rs1: x{}", rs1 as u8);
    println!("   - rs2: x{}", rs2 as u8);
    println!("   - Inmediato: {:#X}", imm);
    println!("   - Func7: {:#07b}", func7);
}

fn sign_ext20(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 20 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x80000) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0xFFFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}

fn sign_ext21(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 21 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x100000) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0x1FFFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}


fn disassemble(inst: u32) -> String {

  //-- Obtener el opcode 
  let mcode = MCode::new(inst);

  //-- y todos los campos de la instrucción  
  let opcode = mcode.opcode();
  let rd: Reg = mcode.rd(); 
  let rs1: Reg = mcode.rs1();
  let imm: i32 = mcode.imm12();
  let offset_jalr: i32 = mcode.imm12();

  let imm20: i32 = get_imm20(inst);
  let offset_jal: i32 = get_offset_jal(inst);
  
  match opcode {
    OpcodeRV::TipoIArith => {
        let inst2: InstructionRV = InstructionRV::from_mcode(inst);
        inst2.to_string()
    },

    OpcodeRV::TipoILoad => {
        let inst2: InstructionRV = InstructionRV::from_mcode(inst);
        inst2.to_string()
    },

    OpcodeRV::TipoR => {
        let inst2: InstructionRV = InstructionRV::from_mcode(inst);
        inst2.to_string()
    },

    OpcodeRV::TipoS => {
        let inst2: InstructionRV = InstructionRV::from_mcode(inst);
        inst2.to_string()
    },

    OpcodeRV::TipoB => {
        let inst2: InstructionRV = InstructionRV::from_mcode(inst);
        inst2.to_string()
    },

    OpcodeRV::TipoULui => {
        let inst2: InstructionRV = InstructionRV::from_mcode(inst);
        inst2.to_string()
    },

    OpcodeRV::TipoUAuipc => {
      format!("auipc x{}, {:#07X}", rd as u8, imm20 & 0xFFFFF)
    },

    OpcodeRV::TipoJJal => {
      format!("jal x{}, {}", rd as u8, offset_jal)
    },

    OpcodeRV::TipoJJalr => {
      //-- Instrucción jalr
      format!("jalr x{}, {}(x{})", rd as u8, offset_jalr, rs1 as u8)
    },

    OpcodeRV::TipoEcallEbreak => {
      //-- Instrucción ecall o ebreak
      if imm == 0 {
        format!("ecall")
      } else if imm == 1 {
        format!("ebreak")
      } else {
        format!("DESCONOCIDA")
      }
    },

    _ => {
      //-- No es una instrucción tipo I aritmética
      println!("   - Instrucción: DESCONOCIDA");
      print_fields(inst);
      String::from("DESCONOCIDA")
    }
  }

}

fn main1() {
   //-- Instrucciones RISC-V a desensamblar
    let insts = [
        0x00100093, // 🟢addi x1, x0, 1
        0x00111093, // 🟢slli x1, x2, 1
        0x00112093, // 🟢slti x1, x2, 1
        0x00113093, // 🟢sltiu x1, x2, 1
        0x00114093, // 🟢xori x1, x2, 1
        0x00115093, // 🟢srli x1, x2, 1 
        0x00116093, // 🟢ori x1, x2, 1
        0x00117093, // 🟢andi x1, x2, 1
        0x40115093, // 🟢srai x1, x2, 1
        0x00008003, // 🟢lb x0, 0(x1)
        0x00009003, // 🟢lh x0, 0(x1)
        0x0000a003, // 🟢lw x0, 0(x1)
        0x0000b003, // 🟢ld x0, 0(x1)
        0x0000c003, // 🟢lbu x0, 0(x1)
        0x0000d003, // 🟢lhu x0, 0(x1)
        0x0000e003, // 🟢lwu x0, 0(x1)
        0x00208033, // 🟢add x0, x1, x2
        0x40208033, // 🟢sub x0, x1, x2
        0x00209033, // 🟢sll x0, x1, x2
        0x0020a033, // 🟢slt x0, x1, x2
        0x0020b033, // 🟢sltu x0, x1, x2
        0x0020c033, // 🟢xor x0, x1, x2
        0x0020d033, // 🟢srl x0, x1, x2
        0x0020e033, // 🟢or x0, x1, x2
        0x4020d033, // 🟢sra x0, x1, x2
        0x00008023, // 🟢sb x0, 0(x1)
        0xfe219fa3, // 🟢sh x2, -1(x3)
        0x7e42afa3, // 🟢sw x4, 2047(x5)
        0x80533023, // 🟢sd x5, -2048(x6)
        0xfe208ee3, // 🟢beq x1, x2, -4
        0xfe419ce3, // 🟢bne x3, x4, -8
        0xfe62cae3, // 🟢blt x5, x6, -12
        0x0083dc63, // 🟢bge x7, x8, 24
        0x00a4ea63, // 🟢bltu x9, x10, 20
        0x00c5f863, // 🟢bgeu x11, x12, 16
        0x80000337, // 🟢lui x6, 0x80000
        0x08000217, // auipc x4, 0x08000
        0xff1ff26f, // jal x4, -16
        0xfff500e7, // jalr x1, -1(x10)
        0x00000073, // ecall
        0x00100073, // ebreak
    ];


    //-- TODO
    //----- Modo privilegiado
    //-- csrrw
    //-- csrrs
    //-- csrrc
    //-- csrrwi
    //-- csrrsi
    //-- csrrci
    //-- uret
    //-- sret
    //-- mret
    //-- wfi
    //-- fence
    //-- fence.i
    //-- sfence.vma
    //----RV64I
    //-- addiw
    //-- slliw
    //-- srliw
    //-- sraiw
    //-- addw
    //-- subw
    //-- sllw
    //-- srlw
    //-- sraw
    //-- lwu

    for i in 0..insts.len() {

        let machine_code: u32 = insts[i];

        //-- Pasar la instruccion a String
        let inst: String = disassemble(insts[i]);

        //-- Imprimirla!
        println!("🟢 [{machine_code:#010X}]: {inst}");

    }
}

fn main_test1() {

    println!("\n------ TESTING 1.....");
    let inst = [

        //-- Instruciones Tipo I
        InstructionRV::Addi{rd: Reg::X1, rs1: Reg::X0, imm: 1},
        InstructionRV::Slli{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Slti{rd: Reg::X1, rs1: Reg::X2, imm: 1}, 
        InstructionRV::Sltiu{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Xori{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Srli{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Ori{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Andi{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Srai{rd: Reg::X1, rs1: Reg::X2, imm: 1},
        InstructionRV::Lb { rd: Reg::X0, offs: 0, rs1: Reg::X1 },
        InstructionRV::Lh { rd: Reg::X0, offs: 0, rs1: Reg::X1 },
        InstructionRV::Lw { rd: Reg::X0, offs: 0, rs1: Reg::X1 },
        InstructionRV::Ld { rd: Reg::X0, offs: 0, rs1: Reg::X1 },
        InstructionRV::Lbu { rd: Reg::X0, offs: 0, rs1: Reg::X1 },
        InstructionRV::Lhu { rd: Reg::X0, offs: 0, rs1: Reg::X1 },
        InstructionRV::Lwu { rd: Reg::X0, offs: 0, rs1: Reg::X1 },

        //-- Instruciones Tipo R
        InstructionRV::Add { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Sub { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Sll { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Sltu { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Xor { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Srl { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Or { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},
        InstructionRV::Sra { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2},

        //-- Instruciones Tipo S
        InstructionRV::Sb { rs2: Reg::X0, offs: 0, rs1: Reg::X1},
        InstructionRV::Sh { rs2: Reg::X2, offs: -1, rs1: Reg::X3},
        InstructionRV::Sw { rs2: Reg::X4, offs: 2047, rs1: Reg::X5},
        InstructionRV::Sd { rs2: Reg::X5, offs: -2048, rs1: Reg::X6},

        //-- Instructiones Tipo B
        InstructionRV::Beq {rs1: Reg::X1, rs2: Reg::X2, offs: -4},
        InstructionRV::Bne {rs1: Reg::X3, rs2: Reg::X4, offs: -8},
        InstructionRV::Blt {rs1: Reg::X5, rs2: Reg::X6, offs: -12},
        InstructionRV::Bge {rs1: Reg::X7, rs2: Reg::X8, offs: 24},
        InstructionRV::Beq {rs1: Reg::X9, rs2: Reg::X10, offs: 20},
        InstructionRV::Beq {rs1: Reg::X11, rs2: Reg::X12, offs: 16},

        //-- Instrucciones Tipo U
        InstructionRV::Lui {rd: Reg::X6, imm: 0x80000},

        //0x08000217, // auipc x4, 0x08000
        //0xff1ff26f, // jal x4, -16
        //0x00000073, // ecall
        //0x00100073, // ebreak
    ];

    for i in 0..inst.len() {
        //-- Imprimir la instrucción
        println!("🟢 {}", inst[i].to_string());
    }

}

fn main_test2() {
    println!("\n------ TESTING 2.....");
    let mcode = [
        0x00100093, //-- addi x1, x0, 1
        0x00111093, //-- slli x1, x2, 1
        0x00112093, //-- slti x1, x2, 1
        0x00113093, //-- sltiu x1, x2, 1
        0x00114093, //-- xori x1, x2, 1
        0x00115093, //-- srli x1, x2, 1
        0x00116093, //-- ori x1, x2, 1
        0x00117093, //-- andi x1, x2, 1
        0x40115093, //-- srai x1, x2, 1
        0x00008003, //-- lb x0, 0(x1)
        0x00009003, //-- lh x0, 0(x1)
        0x0000a003, //-- lw x0, 0(x1)
        0x0000b003, //-- ld x0, 0(x1)
        0x0000c003, //-- lbu x0, 0(x1)
        0x0000d003, //-- lhu x0, 0(x1)
        0x0000e003, //-- lwu x0, 0(x1)
        0x00208033, //-- add x0, x1, x2
        0x40208033, //-- sub x0, x1, x2
        0x00209033, //-- sll x0, x1, x2
        0x0020a033, //-- slt x0, x1, x2
        0x0020b033, //-- sltu x0, x1, x2
        0x0020c033, //-- xor x0, x1, x2
        0x0020d033, //-- srl x0, x1, x2
        0x0020e033, //-- or x0, x1, x2
        0x4020d033, //-- sra x0, x1, x2
        0x00008023, //-- sb x0, 0(x1)
        0xfe219fa3, //-- sh x2, -1(x3)
        0x7e42afa3, //-- sw x4, 2047(x5)
        0x80533023, //-- sd x5, -2048(x6)
        0xfe208ee3, //-- beq x1, x2, -4
        0xfe419ce3, //-- bne x3, x4, -8
        0xfe62cae3, //-- blt x5, x6, -12
        0x0083dc63, //-- bge x7, x8, 24
        0x00a4ea63, //-- bltu x9, x10, 20
        0x00c5f863, //-- bgeu x11, x12, 16
        0x80000337, //-- lui x6, 0x80000
        0x08000217, //-- auipc x4, 0x08000
        0xff1ff26f, //-- jal x4, -16
        0x00000073, //-- ecall
        0x00100073, //-- ebreak
    ];

    

    for i in 0..mcode.len() {
        let inst: InstructionRV = InstructionRV::from_mcode(mcode[i]);
        println!("🟢 [{:#010X}]: {}", mcode[i] as u32, inst.to_string());
    }
}


//────────────────────────────────────────────────
//  PROGRAMA PRINCIPAL
//────────────────────────────────────────────────
fn main() {

    main_test1();
    main_test2();    

    //-- old...
    println!("\n\n--- Main1");
    main1();
}



