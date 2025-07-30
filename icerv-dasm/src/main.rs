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
const FIELD_4B: u32 = 0x0F;  //-- Campo de 4 bits
const FIELD_5B: u32 = 0x1F;  //-- Campo de 5 bits
const FIELD_6B: u32 = 0x3F;  //-- Campo de 6 bits
const FIELD_7B: u32 = 0x7F;  //-- Campo de 7 bits
const FIELD_8B: u32 = 0xFF;  //-- Campo de 8 bits
const FIELD_10B: u32 = 0x3FF; //-- Campo de 10 bits
const FIELD_20B: u32 = 0xFFFFF; //-- Campo de 20 bits
//────────────────────────────────────────────────
//  POSICIONES de LOS CAMPOS
//────────────────────────────────────────────────
const RS2_POS: u8 = 20;  
const FUNC7_POS: u8 = 25;  
const IMM20_POS: u8 = 12; 
const OFFSET10_POS: u8 = 21;
const OFFSET8_POS: u8 = 12;
const OFFSET7_POS: u8 = 25;
const OFFSET6_POS: u8 = 25;
const OFFSET5_POS: u8 = 7;
const OFFSET4_POS: u8 = 8;
const OFFSET1_POS: u8 = 20;
//────────────────────────────────────────────────
//  POSICIONES Bits aislados
//────────────────────────────────────────────────
const BIT5: u32 = 1 << 5;
//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
//  Se calculan desplazando los campos de la anchura correspondiente
//  a la posición del campo
//──────────────────────────────────────────────
const RS2_MASK: u32 = FIELD_5B << RS2_POS;
const FUNC7_MASK: u32 = FIELD_7B << FUNC7_POS;
const IMM20_MASK: u32 = FIELD_20B << IMM20_POS; 
const OFFSET7_MASK: u32 = FIELD_7B << OFFSET7_POS;
const OFFSET8_MASK: u32 = FIELD_8B << OFFSET8_POS; 
const OFFSET10_MASK: u32 = FIELD_10B << OFFSET10_POS;
const OFFSET6_MASK: u32 = FIELD_6B << OFFSET6_POS;
const OFFSET5_MASK: u32 = FIELD_5B << OFFSET5_POS;
const OFFSET4_MASK: u32 = FIELD_4B << OFFSET4_POS;
const OFFSET1_MASK: u32 = FIELD_1B << OFFSET1_POS;

fn get_rs2(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Registro fuente 2 (rs2) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0    
    (inst & RS2_MASK) >> RS2_POS    
}

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

fn get_func7(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Func7 de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & FUNC7_MASK) >> FUNC7_POS
}

fn get_offset_s(inst: u32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Valor del offset para instrucciones s
//────────────────────────────────────────────────
    //-- Extraer el campo offset7
    let offset7: u32 = (inst & OFFSET7_MASK) >> OFFSET7_POS;
    let offset5: u32 = (inst & OFFSET5_MASK) >> OFFSET5_POS;
    let offset: u32 = offset7 << 5 | offset5;

    sign_ext(offset as i32)
}

fn get_offset_b(inst: u32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Valor del offset para instrucciones b
//────────────────────────────────────────────────
    //-- Extraer el bit de signo
    let sign: u32 = (inst & 0x8000_0000) >> 31;

    //-- Extraer el bit 11, de la posicion 7
    let b11: u32 = (inst & 0x0000_0080) >> 7;

    let offset6: u32 = (inst & OFFSET6_MASK) >> OFFSET6_POS;

    let offset4: u32 = (inst & OFFSET4_MASK) >> OFFSET4_POS;

    //-- Construir el offset final a partir de todos sus componentes
    let offset: u32 = (sign << 12) | (b11 << 11) |  
                      (offset6 << 5) | offset4<<1;    
    sign_ext13(offset as i32)
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

    let rs2 = get_rs2(inst);
    let func7 = get_func7(inst);

    //-- Imprimir los campos extraídos
    println!("   - Opcode: {:#4X}", opcode as u32);
    println!("   - rd: x{}", rd as u8);
    println!("   - func3: {:#05b}", func3);
    println!("   - rs1: x{}", rs1 as u8);
    println!("   - rs2: x{}", rs2);
    println!("   - Inmediato: {:#X}", imm);
    println!("   - Func7: {:#07b}", func7);
}

fn sign_ext(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 12 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x800) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0xFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}

fn sign_ext13(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 13 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x1000) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0x1FFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
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


fn inst_type_i_load(func3: u32) -> String {
//────────────────────────────────────────────────
//  Obtener el nombre de la instruccion load
//  de tipo i cuyo codigo func3 es el dado
//  ENTRADA: Codigo func3
//  SALIDA: nemonico
//────────────────────────────────────────────────

    //-- Tabla de las instrucciones aritméticas
    //-- y lógicas con valores inmediatos
    let name = 
    [          //-- Func3
      "lb",    //-- 000
      "lh",    //-- 001
      "lw",    //-- 010
      "ld",    //-- 011
      "lbu",   //-- 100
      "lhu",   //-- 101
      "lwu",   //-- 110
      "xxx",   //-- 111
    ];

    name[func3 as usize].to_string()

}

fn inst_type_r(func7: u32, func3: u32) -> String {
//────────────────────────────────────────────────
//  Obtener el nombre de la instruccion de tipo R
//  a partir de func3 y func7
//  ENTRADA: Codigos func7 y func3
//  SALIDA: nemonico
//────────────────────────────────────────────────
    let name = [

      //-- Bloque 1: Bit 5 de func7 a 0
              //-- Func3    Func7
      "add",  //-- 000     0000000
      "sll",  //-- 001     0000000
      "slt",  //-- 010     0000000
      "sltu", //-- 011     0000000
      "xor",  //-- 100     0000000
      "srl",  //-- 101     0000000
      "or",   //-- 110     0000000
      "and",  //-- 111     0000000

      //-- Bloque 2: Bit 5 de func7 a 1
      "sub",  //-- 000     0100000
      "xxx",  //-- 001     xxxxxxx  No existe
      "xxx",  //-- 010     xxxxxxx  No existe
      "xxx",  //-- 011     xxxxxxx  No existe
      "xxx",  //-- 100     xxxxxxx  No existe
      "sra",  //-- 101     0100000 
      "xxx",  //-- 110     xxxxxxx  No existe
      "xxx",  //-- 111     xxxxxxx  No existe
    ];

    //-- Obtener el bit 5 de func7
    let especial: u32 = func7 & BIT5;

    //-- Moverlo al bit 3
    //-- Si es instruccion especial (sub, sra), entonces
    //-- especial vale 8, de lo contrario 0
    let especial: u32 = especial >> 2;

    //-- Sumar func3 + special para acceder a la tabla

    //-- Devolver el nombre del nemonico
    name[(func3 + especial) as usize].to_string()

}

fn inst_type_s(func3: u32) -> String {
//────────────────────────────────────────────────
//  Obtener el nombre de la instruccion de tipo S
//  a partir de func3
//  ENTRADA: Codigos func3
//  SALIDA: nemonico
//────────────────────────────────────────────────
    let name = [
             //-- func3
      "sb",  //-- 000
      "sh",  //-- 001
      "sw",  //-- 010
      "sd",  //-- 011
    ];

    name[func3 as usize].to_string()
}

fn inst_type_b(func3: u32) -> String {
//────────────────────────────────────────────────
//  Obtener el nombre de la instruccion de tipo B
//  a partir de func3
//  ENTRADA: Codigos func3
//  SALIDA: nemonico
//────────────────────────────────────────────────
    let name = [
             //-- func3
      "beq",  //-- 000
      "bne",  //-- 001
      "xxx",  //-- 010
      "xxx",  //-- 011
      "blt",  //-- 100
      "bge",  //-- 101
      "bltu", //-- 110
      "bgeu", //-- 111
    ];

    name[func3 as usize].to_string()
}

fn disassemble(inst: u32) -> String {

  //-- Obtener el opcode 
  let mcode = MCode::new(inst);

  //-- y todos los campos de la instrucción  
  let opcode = mcode.opcode();
  let rd: Reg = mcode.rd(); 
  let rs1: Reg = mcode.rs1();
  let func3: u32 = mcode.func3();
  let imm: i32 = mcode.imm12();
  let offset_jalr: i32 = mcode.imm12();

  let func7: u32 = get_func7(inst);
  let rs2 = get_rs2(inst);
  let offset:i32 = get_offset_s(inst);
  let imm20: i32 = get_imm20(inst);
  let offset_b: i32 = get_offset_b(inst);
  let offset_jal: i32 = get_offset_jal(inst);
  
  match opcode {
    OpcodeRV::TipoIArith => {
      let inst2: InstructionRV = InstructionRV::from_mcode(inst);
      inst2.to_string()
    },

    OpcodeRV::TipoILoad => {
      //-- Nombre de la instrucción
      let name: String = inst_type_i_load(func3);

      format!("{} x{}, {}(x{})", name, rd as u8, imm, rs1 as u8)
    },

    OpcodeRV::TipoR => {
      let name: String = inst_type_r(func7, func3);
      format!("{} x{}, x{}, x{}", name, rd as u8, rs1 as u8, rs2)
    },

    OpcodeRV::TipoS => {
      //-- Nombre de la instrucción
      let name: String = inst_type_s(func3);
      format!("{} x{}, {}(x{})", name, rs2, offset, rs1 as u8)
    },

    OpcodeRV::TipoB => {
      //-- Nombre de la instrucción
      let name: String = inst_type_b(func3);
      format!("{} x{}, x{}, {}", name, rs1 as u8, rs2, offset_b)
    },

    OpcodeRV::TipoULui => {
      format!("lui x{}, {:#07X}", rd as u8, imm20 & 0xFFFFF)
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
        0x0000e003, // lwu x0, 0(x1)
        0x00208033, // add x0, x1, x2
        0x40208033, // sub x0, x1, x2
        0x00209033, // sll x0, x1, x2
        0x0020a033, // slt x0, x1, x2
        0x0020b033, // sltu x0, x1, x2
        0x0020c033, // xor x0, x1, x2
        0x0020d033, // srl x0, x1, x2
        0x0020e033, // or x0, x1, x2
        0x4020d033, // sra x0, x1, x2
        0x00008023, // sb x0, 0(x1)
        0xfe219fa3, // sh x2, -1(x3)
        0x7e42afa3, // sw x4, 2047(x5)
        0x80533023, // sd x5, -2048(x6)
        0xfe208ee3, // beq x1, x2, -4
        0xfe419ce3, // bne x3, x4, -8
        0xfe62cae3, // blt x5, x6, -12
        0x0083dc63, // bge x7, x8, 24
        0x00a4ea63, // bltu x9, x10, 20
        0x00c5f863, // bgeu x11, x12, 16
        0x80000337, // lui x6, 0x80000
        0x08000217, // auipc x4, 0x08000
        0xff1ff26f, // jal x4, -16
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
        0x0000d003, // lhu x0, 0(x1)
        0x0000e003, // lwu x0, 0(x1)
        
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


//────────────────────────────────────────────────
//  TESTS
//────────────────────────────────────────────────
#[test]
fn test_get_rs2() {
  //-- Test de la función get_rs2
  //--                   func7  rs2   rs1  func3 rd    opcode
  assert_eq!(get_rs2(0b_0000000_00000_00000_000_00000_0000000), 0);
  assert_eq!(get_rs2(0b_0000000_00001_00000_000_00000_0000000), 1);
  assert_eq!(get_rs2(0b_0000000_00010_00000_000_00000_0000000), 2);
  assert_eq!(get_rs2(0b_0000000_00100_00000_000_00000_0000000), 4);
  assert_eq!(get_rs2(0b_0000000_01000_00000_000_00000_0000000), 8);
  assert_eq!(get_rs2(0b_0000000_10000_00000_000_00000_0000000), 16);
  assert_eq!(get_rs2(0b_0000000_10001_00000_000_00000_0000000), 17);
  assert_eq!(get_rs2(0b_0000000_11111_00000_000_00000_0000000), 31);
}

#[test]
fn test_get_func7() {
  //-- Test de la función get_func7
  //--                   func7  rs2   rs1  func3 rd    opcode
  assert_eq!(get_func7(0b_0000000_00000_00000_000_00000_0000000), 0b0000000);
  assert_eq!(get_func7(0b_0000001_00000_00000_000_00000_0000000), 0b0000001);
  assert_eq!(get_func7(0b_0000010_00000_00000_000_00000_0000000), 0b0000010);
  assert_eq!(get_func7(0b_0000100_00000_00000_000_00000_0000000), 0b0000100);
  assert_eq!(get_func7(0b_0001000_00000_00000_000_00000_0000000), 0b0001000);
  assert_eq!(get_func7(0b_0010000_00000_00000_000_00000_0000000), 0b0010000);
  assert_eq!(get_func7(0b_0100000_00000_00000_000_00000_0000000), 0b0100000);
  assert_eq!(get_func7(0b_1000000_00000_00000_000_00000_0000000), 0b1000000);
  assert_eq!(get_func7(0b_1111111_00000_00000_000_00000_0000000), 0b1111111);
}

#[test]
fn test_sign_ext() {
  //-- Test de la función sign_ext
  assert_eq!(sign_ext(0x000), 0);
  assert_eq!(sign_ext(0x001), 1);
  assert_eq!(sign_ext(0x7FF), 2047);  //-- 0x7FF
  assert_eq!(sign_ext(0x800), -2048); //-- 0x800
  assert_eq!(sign_ext(0xFFF), -1);
  assert_eq!(sign_ext(0x7FF_FFFF), -1);
}


