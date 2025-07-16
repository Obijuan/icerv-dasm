//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────

//-- Instrucciones RV32I
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

//────────────────────────────────────────────────
//  CONSTANTES PARA ACCESO A LA ISA DEL RISCV   
//────────────────────────────────────────────────
//  Definir anchura de campos. Estos campos son los que luego se llevan
//  a la posición concreta en la instrucción, para calcular la máscara
//  con la que extraer el campo
//───────────────────────────
//  ANCHURAS de LOS CAMPOS
//───────────────────────────
const FIELD_3B: u32 = 0x07;  //-- Campo de 3 bits de ancho
const FIELD_4B: u32 = 0x0F;  //-- Campo de 4 bits
const FIELD_5B: u32 = 0x1F;  //-- Campo de 5 bits
const FIELD_6B: u32 = 0x3F;  //-- Campo de 6 bits
const FIELD_7B: u32 = 0x7F;  //-- Campo de 7 bits
const FIELD_12B: u32 = 0xFFF;  //-- Campo de 12 bits
const FIELD_20B: u32 = 0xFFFFF; //-- Campo de 20 bits
//────────────────────────────────────────────────
//  POSICIONES de LOS CAMPOS
//────────────────────────────────────────────────
const OPCODE_POS: u8 = 0;  
const RD_POS: u8 = 7;
const FUNC3_POS: u8 = 12;  
const RS1_POS: u8 = 15;  
const RS2_POS: u8 = 20;  
const FUNC7_POS: u8 = 25;  
const IMM12_POS: u8 = 20; 
const IMM20_POS: u8 = 12; 
const OFFSET7_POS: u8 = 25;
const OFFSET6_POS: u8 = 25;
const OFFSET5_POS: u8 = 7;
const OFFSET4_POS: u8 = 8;
//────────────────────────────────────────────────
//  POSICIONES Bits aislados
//────────────────────────────────────────────────
const BIT10: u32 = 1 << 10;
const BIT5: u32 = 1 << 5;
//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
//  Se calculan desplazando los campos de la anchura correspondiente
//  a la posición del campo
//──────────────────────────────────────────────
const OPCODE_MASK: u32 = FIELD_7B << OPCODE_POS; 
const RD_MASK: u32 = FIELD_5B << RD_POS;  
const FUNC3_MASK: u32 = FIELD_3B << FUNC3_POS;  
const RS1_MASK: u32 = FIELD_5B << RS1_POS;  
const RS2_MASK: u32 = FIELD_5B << RS2_POS;
const FUNC7_MASK: u32 = FIELD_7B << FUNC7_POS;
const IMM12_MASK: u32 = FIELD_12B << IMM12_POS;  
const IMM20_MASK: u32 = FIELD_20B << IMM20_POS; 
const OFFSET7_MASK: u32 = FIELD_7B << OFFSET7_POS;
const OFFSET6_MASK: u32 = FIELD_6B << OFFSET6_POS;
const OFFSET5_MASK: u32 = FIELD_5B << OFFSET5_POS;
const OFFSET4_MASK: u32 = FIELD_4B << OFFSET4_POS;
//────────────────────────────────────────────────
//  DEFINICION DE LOS OPCODES
//────────────────────────────────────────────────
//  Instrucciones tipo-I
//────────────────────────────
//-- Estas instrucciones se dividen a su vez en dos grupos:
//  - Instrucciones aritméticas (ADDI, ANDI, ORI,...)
//  - Instrucciones de carga (LW, LH, LB,...)
const OPCODE_I_ARITH: u32 = 0x13;   //-- ADDI: addi rd, rs1, imm12
const OPCODE_I_LOAD: u32 = 0x03;    //-- LW: lw rd, imm12(rs1)
//  Instrucciones tipo-R
//────────────────────────────
const OPCODE_R: u32 = 0b_01100_11;  //-- 0x33
//  Instrucciones tipo-S
//────────────────────────────
const OPCODE_S: u32 = 0b_01000_11;  //-- 0x23
//  Instrucciones tipo-B
//────────────────────────────
const OPCODE_B: u32 = 0b_11000_11; //-- 0x63
//  Instruccion tipo-U: LUI
//────────────────────────────
const OPCODE_U_LUI: u32 = 0b_01101_11; //--0x37 
//  Instruccion tipo-U: AUIPC
//────────────────────────────
const OPCODE_U_AUIPC: u32 = 0b_00101_11; //--0x17 

fn get_opcode(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Opcode de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & OPCODE_MASK) >> OPCODE_POS
}

fn get_rd(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Registro destino (rd) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & RD_MASK) >> RD_POS
}

fn get_func3(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Func3 de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & FUNC3_MASK) >> FUNC3_POS
}

fn get_rs1(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V          
// Salida: Registro fuente 1 (rs1) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & RS1_MASK) >> RS1_POS
}

fn get_rs2(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Registro fuente 2 (rs2) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0    
    (inst & RS2_MASK) >> RS2_POS    
}

fn get_imm12(inst: u32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Inmediato de 12 bits de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  let imm12: u32 = (inst & IMM12_MASK) >> IMM12_POS;

  //-- Convertir el valor a i32 para manejar el signo
  //-- y devolverlo!
  sign_ext(imm12 as i32)
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

fn print_fields(inst: u32) {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Imprime los campos de la instrucción
//────────────────────────────────────────────────

    //-- Extraer los campos de la instrucción
    let opcode = get_opcode(inst);
    let rd = get_rd(inst);
    let func3 = get_func3(inst);
    let rs1 = get_rs1(inst);
    let rs2 = get_rs2(inst);
    let imm = get_imm12(inst);
    let func7 = get_func7(inst);

    //-- Imprimir los campos extraídos
    println!("   - Opcode: {:#4X}", opcode);
    println!("   - rd: x{}", rd);
    println!("   - func3: {:#05b}", func3);
    println!("   - rs1: x{}", rs1);
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



fn is_type_i(opcode: u32) -> bool {
//────────────────────────────────────────────────
// Entrada: opcode RISC-V
// Salida: true si es una instrucción tipo I
//────────────────────────────────────────────────
    //-- Las instrucciones de tipo I son bien las de tipo
    //-- aritmético (ADDI, ANDI, ORI,...) o las de LOAD (LW, LH, LB,...)
    if opcode == OPCODE_I_ARITH || opcode == OPCODE_I_LOAD {
        true
    } else {
        false
    }
}

fn is_type_i_arith(opcode: u32) -> bool {
    if opcode == OPCODE_I_ARITH {
        true
    }
    else {
        false
    }
}

fn is_type_i_load(opcode: u32) -> bool {
    if opcode == OPCODE_I_LOAD {
      true
    }
    else {
      false
    }
}

fn is_type_u_lui(opcode: u32) -> bool {
    if opcode == OPCODE_U_LUI {
        true
    }
    else {
        false
    }
}

fn is_type_u_auipc(opcode: u32) -> bool {
    if opcode == OPCODE_U_AUIPC {
        true
    }
    else {
        false
    }
}

fn is_type_r(opcode: u32) -> bool {
    if opcode == OPCODE_R {
      true
    }
    else {
      false
    }
}

fn is_type_s(opcode: u32) -> bool {
    if opcode == OPCODE_S {
      true
    }
    else {
      false
    }
}

fn is_type_b(opcode: u32) -> bool {
    if opcode == OPCODE_B {
      true
    }
    else {
      false
    }
}


fn inst_type_i_arith(func3: u32, imm: i32) -> String {
//────────────────────────────────────────────────
//  Obtener el nombre de la instruccion aritmetica
//  de tipo i cuyo codigo func3 es el dado
//  ENTRADA: Codigo func3
//  SALIDA: nemonico
//────────────────────────────────────────────────

    //-- Tabla de las instrucciones aritméticas
    //-- y lógicas con valores inmediatos
    let name = 
    [          //-- Func3
      "addi",  //-- 000
      "slli",  //-- 001
      "slti",  //-- 010
      "sltiu", //-- 011
      "xori",  //-- 100
      "srli",  //-- 101  srli, srai (Bit 30 a 1)
      "ori",   //-- 110
      "andi",  //-- 111
    ];

    //-- Caso especial: srli y srai tienen el mismo codigo func3
    //-- Se diferenencias por el bit 30 del opcode (bit 10 del valor imm)
    let bit_srali:u32 = imm as u32 >> 10; 

    //-- Caso especial
    if (bit_srali==1) && (func3==0b101) {
      "srai".to_string()

    //-- Caso general
    } else {
      //-- Devolver la cadena a partir del código
      name[func3 as usize].to_string()
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
//────────────────────────────────────────────────
// Desensamblar una instruccion en codigo maquina
// Entrada: Instrucción en codigo maquina RISC-V
// Salida: Cadena con la instrucción en ensamblador
//────────────────────────────────────────────────

    //-- Extraer el opcode de la instrucción  
    let opcode = get_opcode(inst);

    //-- Comprobar el tipo de instrucción que es, según el opcode
    if is_type_i(opcode) {

        //-- Sabemos el formato, podemos obtener todos los campos
        let func3 = get_func3(inst);

        //-- Distinguir entre el tipo I de instruccion:
        //--   - Instrucciones aritmeticas (ADDI, ANDI, ORI,...)
        //--   - Instrucciones de carga (LW, LH, LB,...)
        if is_type_i_arith(opcode) {

            let rd = get_rd(inst);
            let rs1 = get_rs1(inst);
            let imm = get_imm12(inst) as i32;

            //-- Nombre de la instruccion            
            let name = inst_type_i_arith(func3, imm);

            //-- Caso especial: srai
            //-- El 10 de imm está a 1 (en caso de srai)
            //-- Este bit hay que ponerlo a 0
            let imm2: i32 = if (imm as u32 & BIT10==0x400) && (func3==0b101) {
                (imm as u32 & !BIT10) as i32
            } else {
              imm
            };

            //-- Devolver la instruccion completa en ensamblador
            format!("{} x{}, x{}, {}", name, rd, rs1, imm2)

        } else if is_type_i_load(opcode) {
            let rd = get_rd(inst);
            let rs1 = get_rs1(inst);
            let imm = get_imm12(inst);

            //-- Nombre de la instrucción
            let name: String = inst_type_i_load(func3);

            format!("{} x{}, {}(x{})", name, rd, imm, rs1)
        } else {
            println!("   - Instrucción: DESCONOCIDA");
            print_fields(inst);
            String::from("DESCONOCIDA")
        }

    } else if is_type_r(opcode) {

        //-- Sabemos el formato, podemos obtener todos los campos
        let rd = get_rd(inst);
        let rs1 = get_rs1(inst);
        let rs2 = get_rs2(inst);
        let func3 = get_func3(inst);
        let func7: u32 = get_func7(inst);

        let name: String = inst_type_r(func7, func3);

        format!("{} x{}, x{}, x{}", name, rd, rs1, rs2)
    } else if is_type_s(opcode) {
        //-- Sabemos el formato, podemos obtener todos los campos
        let rs1 = get_rs1(inst);
        let rs2 = get_rs2(inst);
        let func3 = get_func3(inst);
        let offset:i32 = get_offset_s(inst);
        //-- TODO: Valores inmediatos...

        let name: String = inst_type_s(func3);

        format!("{} x{}, {}(x{})", name, rs2, offset, rs1)
    } else if is_type_b(opcode) {

        let rs1 = get_rs1(inst);
        let rs2 = get_rs2(inst);
        let func3 = get_func3(inst);

        let name: String = inst_type_b(func3);
        let offset: i32 = get_offset_b(inst);

        format!("{} x{}, x{}, {}", name, rs1, rs2, offset)
    } else if is_type_u_lui(opcode) {

        let imm20: i32 = get_imm20(inst);
        let rd = get_rd(inst);
        format!("lui x{}, {:#07X}", rd, imm20 & 0xFFFFF)
    } else if is_type_u_auipc(opcode) {
        let imm20: i32 = get_imm20(inst);
        let rd = get_rd(inst);
        format!("auipc x{}, {:#07X}", rd, imm20 & 0xFFFFF)
    }
    else {
        println!("   - Instrucción: DESCONOCIDA");
        print_fields(inst);
        String::from("DESCONOCIDA")
    }
}


//────────────────────────────────────────────────
//  PROGRAMA PRINCIPAL
//────────────────────────────────────────────────
fn main() {

    //-- Instrucciones RISC-V a desensamblar
    let insts = [
        0x40115093, // srai x1, x2, 1
        0x00100093, // addi x1, x0, 1
        0x00111093, // slli x1, x2, 1
        0x00112093, // slti x1, x2, 1
        0x00113093, // sltiu x1, x2, 1
        0x00114093, // xori x1, x2, 1
        0x00115093, // srli x1, x2, 1 
        0x00116093, // ori x1, x2, 1
        0x00117093, // andi x1, x2, 1
        0x40115093, // srai x1, x2, 1
        0x00008003, // lb x0, 0(x1)
        0x0000c003, // lbu x0, 0(x1)
        0x00009003, // lh x0, 0(x1)
        0x0000a003, // lw x0, 0(x1)
        0x0000b003, // ld x0, 0(x1)
        0x0000d003, // lhu x0, 0(x1)
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
];

    //-- TODO
    //----- Tipo J
    //-- jal:   0b_11011_11
    //-- jalr:  0b_11001_11
    //----- Llamadas al sistema
    //-- ecall
    //-- ebreak
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


//────────────────────────────────────────────────
//  TESTS
//────────────────────────────────────────────────

#[test]
fn test_get_opcode() {
    //-- Test de la función get_opcode

    //-- Instrucciones reales
    assert_eq!(get_opcode(0x0000_0013), 0x13);
    assert_ne!(get_opcode(0x0000_0013), 0x00);
    assert_eq!(get_opcode(0x0aa0_0093), 0x13);

    //-- Instrucciones inventadas
    assert_eq!(get_opcode(0xffff_ffff), 0x7f);
    assert_eq!(get_opcode(0x0000_0000), 0x00);
    assert_eq!(get_opcode(0xaaaa_aaaa), 0x2a);
    assert_eq!(get_opcode(0x0000_0000_0000_0000__0000_00000_0000001), 0x01);
    assert_eq!(get_opcode(0x0000_0000 | 0b0000001), 0x01);
}

#[test]
fn test_get_rd() {
    //-- Test de la función get_rd

    //-- Instrucciones reales
    assert_eq!(get_rd(0x0000_0013), 0);
    assert_eq!(get_rd(0x0aa0_0093), 1);

    //-- Instrucciones inventandas
    assert_eq!(get_rd(0x0000_0000 | 0b00000_0000000), 0);
    assert_eq!(get_rd(0x0000_0000 | 0b00001_0000000), 1);
    assert_eq!(get_rd(0xFFFF_0000 | 0b00010_0000000), 2); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b00100_0000000), 4); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b01000_0000000), 8); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b10000_0000000), 16);
    assert_eq!(get_rd(0xFFFF_0000 | 0b10001_0000000), 17); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b11111_0000000), 31); 
}

#[test]
fn test_get_func3() {
  //-- Test de la función get_func3

  assert_eq!(get_func3(0b_0000000_00000_00000_000_00000_0000000), 0b000);
  assert_eq!(get_func3(0b_0000000_00000_00000_001_00000_0000000), 0b001);
  assert_eq!(get_func3(0b_0000000_00000_00000_010_00000_0000000), 0b010);
  assert_eq!(get_func3(0b_0000000_00000_00000_100_00000_0000000), 0b100);
  assert_eq!(get_func3(0b_0000000_00000_00000_111_00000_0000000), 0b111);
}

#[test]
fn test_get_rs1() {
  //-- Test de la función get_rs1 

  //--                   func7  rs2   rs1  func3 rd    opcode
  assert_eq!(get_rs1(0b_0000000_00000_00000_000_00000_0000000), 0);
  assert_eq!(get_rs1(0b_0000000_00000_00001_000_00000_0000000), 1);
  assert_eq!(get_rs1(0b_0000000_00000_00010_000_00000_0000000), 2);
  assert_eq!(get_rs1(0b_0000000_00000_00100_000_00000_0000000), 4);
  assert_eq!(get_rs1(0b_0000000_00000_01000_000_00000_0000000), 8);
  assert_eq!(get_rs1(0b_0000000_00000_10000_000_00000_0000000), 16);
  assert_eq!(get_rs1(0b_0000000_00000_10001_000_00000_0000000), 17);
  assert_eq!(get_rs1(0b_0000000_00000_11111_000_00000_0000000), 31);
}

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
fn test_get_imm12() {
  //-- Test de la función get_imm12
  //--                    ----imm12-----   rs1  func3  rd    opcode
  assert_eq!(get_imm12(0b_0000_0000_0000__00000__000__00000__0000000), 0b0000_0000_0000);
  assert_eq!(get_imm12(0b_0000_0000_0001__00000__000__00000__0000000), 0x001);
  assert_eq!(get_imm12(0x001_0_0000), 0x001); 
  assert_eq!(get_imm12(0x002_0_0000), 0x002);
  assert_eq!(get_imm12(0x004_0_0000), 0x004);
  assert_eq!(get_imm12(0x008_0_0000), 0x008);
  assert_eq!(get_imm12(0x010_0_0000), 0x010);
  assert_eq!(get_imm12(0x020_0_0000), 0x020);
  assert_eq!(get_imm12(0x040_0_0000), 0x040);
  assert_eq!(get_imm12(0x080_0_0000), 0x080);
  assert_eq!(get_imm12(0x100_0_0000), 0x100);
  assert_eq!(get_imm12(0x200_0_0000), 0x200);
  assert_eq!(get_imm12(0x400_0_0000), 0x400);

  //-- Pruebs de signo
  assert_eq!(get_imm12(0x800_0_0000), 0xFFFF_F800u32 as i32); //-- -2048
  assert_eq!(get_imm12(0xFFF_0_0000), 0xFFFF_FFFFu32 as i32); //-- -1
  assert_eq!(get_imm12(0xFFF_FFFFF), -1); 
  assert_eq!(get_imm12(0x800_FFFFF), -2048);
  assert_eq!(get_imm12(0x7FF_FFFFF), 2047);
  assert_eq!(get_imm12(0xFFE_FFFFF), -2);
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

#[test]
fn test_is_type_i() {
  //-- Test de la función is_type_i

  //-- Tipos correctos
  assert!(is_type_i(OPCODE_I_ARITH));
  assert!(is_type_i(OPCODE_I_LOAD));
  assert!(is_type_i(0x13));  //-- ADDI
  assert!(is_type_i(0x03));  //-- LW

  //-- Tipos incorrectos
  assert!(!is_type_i(0x00));
  assert!(!is_type_i(0x01));
  assert!(!is_type_i(0x02));
  assert!(!is_type_i(0x04));
  assert!(!is_type_i(0xFF));
}

#[test]
fn test_disassemble_addi() {
    //-- Test de la funcion disassemble
    //-- Instrucciones addi 

    assert_eq!(disassemble(0x00000013), "addi x0, x0, 0");
    assert_eq!(disassemble(0x00100093), "addi x1, x0, 1");
    assert_eq!(disassemble(0x00200113), "addi x2, x0, 2");
    assert_eq!(disassemble(0xfff00193), "addi x3, x0, -1");
    assert_eq!(disassemble(0x7ff00213), "addi x4, x0, 2047");
    assert_eq!(disassemble(0x00308f93), "addi x31, x1, 3");
    assert_eq!(disassemble(0x00410413), "addi x8, x2, 4");
    assert_eq!(disassemble(0x00820813), "addi x16, x4, 8");
    assert_eq!(disassemble(0x01040893), "addi x17, x8, 16");
    assert_eq!(disassemble(0xff040893), "addi x17, x8, -16");
    assert_eq!(disassemble(0x80040893), "addi x17, x8, -2048");
    assert_eq!(disassemble(0x0aa00093), "addi x1, x0, 170");    
}

#[test]
fn test_disassemble_slli() {
    //-- Test de la funcion disassemble
    //-- Instrucciones slli

    assert_eq!(disassemble(0x00111093), "slli x1, x2, 1");
    assert_eq!(disassemble(0x00001013), "slli x0, x0, 0");
    assert_eq!(disassemble(0x00209f93), "slli x31, x1, 2");
    assert_eq!(disassemble(0x00411f13), "slli x30, x2, 4");
    assert_eq!(disassemble(0x00819e93), "slli x29, x3, 8");
    assert_eq!(disassemble(0x01021e13), "slli x28, x4, 16");
    assert_eq!(disassemble(0x01129d93), "slli x27, x5, 17");
    assert_eq!(disassemble(0x01e31d13), "slli x26, x6, 30");
    assert_eq!(disassemble(0x01f39c93), "slli x25, x7, 31");
}

#[test]
fn test_disassemble_slti() {
    assert_eq!(disassemble(0x00112093), "slti x1, x2, 1");
    assert_eq!(disassemble(0x00002013), "slti x0, x0, 0");
    assert_eq!(disassemble(0x0020af93), "slti x31, x1, 2");
    assert_eq!(disassemble(0x00412f13), "slti x30, x2, 4");
    assert_eq!(disassemble(0x0081ae93), "slti x29, x3, 8");
    assert_eq!(disassemble(0x01022e13), "slti x28, x4, 16");
    assert_eq!(disassemble(0x0112ad93), "slti x27, x5, 17");
    assert_eq!(disassemble(0x01e32d13), "slti x26, x6, 30");  
    assert_eq!(disassemble(0x01f3ac93), "slti x25, x7, 31");  
}

#[test]
fn test_disassemble_sltiu()
{
    assert_eq!(disassemble(0x00113093), "sltiu x1, x2, 1");
    assert_eq!(disassemble(0x00003013), "sltiu x0, x0, 0");
    assert_eq!(disassemble(0x0020bf93), "sltiu x31, x1, 2");
    assert_eq!(disassemble(0x00413f13), "sltiu x30, x2, 4");
    assert_eq!(disassemble(0x0081be93), "sltiu x29, x3, 8");
    assert_eq!(disassemble(0x01023e13), "sltiu x28, x4, 16");
    assert_eq!(disassemble(0x0112bd93), "sltiu x27, x5, 17");
    assert_eq!(disassemble(0x01e33d13), "sltiu x26, x6, 30");
    assert_eq!(disassemble(0x01f3bc93), "sltiu x25, x7, 31");
}

#[test]
fn test_disassemble_xori() {
    assert_eq!(disassemble(0x00114093), "xori x1, x2, 1");
    assert_eq!(disassemble(0x00004013), "xori x0, x0, 0");
    assert_eq!(disassemble(0x0020cf93), "xori x31, x1, 2");
    assert_eq!(disassemble(0x00414f13), "xori x30, x2, 4");
    assert_eq!(disassemble(0x0081ce93), "xori x29, x3, 8");
    assert_eq!(disassemble(0x01024e13), "xori x28, x4, 16");
    assert_eq!(disassemble(0x0112cd93), "xori x27, x5, 17");
    assert_eq!(disassemble(0x01e34d13), "xori x26, x6, 30");
    assert_eq!(disassemble(0x01f3cc93), "xori x25, x7, 31");
}

#[test]
fn test_disassemble_srli() {
    assert_eq!(disassemble(0x00115093), "srli x1, x2, 1"); 
    assert_eq!(disassemble(0x00005013), "srli x0, x0, 0");
    assert_eq!(disassemble(0x0020df93), "srli x31, x1, 2");
    assert_eq!(disassemble(0x00415f13), "srli x30, x2, 4");
    assert_eq!(disassemble(0x0081de93), "srli x29, x3, 8");
    assert_eq!(disassemble(0x01025e13), "srli x28, x4, 16");
    assert_eq!(disassemble(0x0112dd93), "srli x27, x5, 17");
    assert_eq!(disassemble(0x01e35d13), "srli x26, x6, 30");
    assert_eq!(disassemble(0x01f3dc93), "srli x25, x7, 31");
}

#[test]
fn test_disassemble_ori() {

    assert_eq!(disassemble(0x00116093), "ori x1, x2, 1");
    assert_eq!(disassemble(0x00006013), "ori x0, x0, 0");
    assert_eq!(disassemble(0x0020ef93), "ori x31, x1, 2");
    assert_eq!(disassemble(0x00416f13), "ori x30, x2, 4");
    assert_eq!(disassemble(0x0081ee93), "ori x29, x3, 8");
    assert_eq!(disassemble(0x01026e13), "ori x28, x4, 16");
    assert_eq!(disassemble(0x0112ed93), "ori x27, x5, 17");
    assert_eq!(disassemble(0x01e36d13), "ori x26, x6, 30");
    assert_eq!(disassemble(0x01f3ec93), "ori x25, x7, 31");
}


#[test]
fn test_disassemble_andi() {
    assert_eq!(disassemble(0x00117093), "andi x1, x2, 1");
    assert_eq!(disassemble(0x00007013), "andi x0, x0, 0");
    assert_eq!(disassemble(0x0020ff93), "andi x31, x1, 2");
    assert_eq!(disassemble(0x00417f13), "andi x30, x2, 4");
    assert_eq!(disassemble(0x0081fe93), "andi x29, x3, 8");
    assert_eq!(disassemble(0x01027e13), "andi x28, x4, 16");
    assert_eq!(disassemble(0x0112fd93), "andi x27, x5, 17");
    assert_eq!(disassemble(0x01e37d13), "andi x26, x6, 30");
    assert_eq!(disassemble(0x01f3fc93), "andi x25, x7, 31");
}

#[test]
fn test_disassemble_srai() {
    assert_eq!(disassemble(0x40115093), "srai x1, x2, 1");
    assert_eq!(disassemble(0x40005013), "srai x0, x0, 0");
    assert_eq!(disassemble(0x4020df93), "srai x31, x1, 2");
    assert_eq!(disassemble(0x40415f13), "srai x30, x2, 4");
    assert_eq!(disassemble(0x4081de93), "srai x29, x3, 8");
    assert_eq!(disassemble(0x41025e13), "srai x28, x4, 16");
    assert_eq!(disassemble(0x4112dd93), "srai x27, x5, 17");
    assert_eq!(disassemble(0x41e35d13), "srai x26, x6, 30");
    assert_eq!(disassemble(0x41f3dc93), "srai x25, x7, 31");
}

#[test]
fn test_disassemble_lbu() {
    assert_eq!(disassemble(0x0000c003), "lbu x0, 0(x1)");
    assert_eq!(disassemble(0x00114083), "lbu x1, 1(x2)");
    assert_eq!(disassemble(0x0021c103), "lbu x2, 2(x3)");
    assert_eq!(disassemble(0x00424203), "lbu x4, 4(x4)");
    assert_eq!(disassemble(0x0082c283), "lbu x5, 8(x5)");
    assert_eq!(disassemble(0xfff34303), "lbu x6, -1(x6)");
    assert_eq!(disassemble(0x8003c383), "lbu x7, -2048(x7)");
    assert_eq!(disassemble(0xffe44403), "lbu x8, -2(x8)");
    assert_eq!(disassemble(0x7ff4c483), "lbu x9, 2047(x9)");
}

#[test]
fn test_disassemble_lb() {
    assert_eq!(disassemble(0x00008003), "lb x0, 0(x1)");
    assert_eq!(disassemble(0x00110083), "lb x1, 1(x2)");
    assert_eq!(disassemble(0x00218103), "lb x2, 2(x3)");
    assert_eq!(disassemble(0x00420203), "lb x4, 4(x4)");
    assert_eq!(disassemble(0x00828283), "lb x5, 8(x5)");
    assert_eq!(disassemble(0xfff30303), "lb x6, -1(x6)");
    assert_eq!(disassemble(0x80038383), "lb x7, -2048(x7)");
    assert_eq!(disassemble(0xffe40403), "lb x8, -2(x8)");
    assert_eq!(disassemble(0x7ff48483), "lb x9, 2047(x9)");
}

#[test]
fn test_disassemble_lh() {
    assert_eq!(disassemble(0x00009003), "lh x0, 0(x1)");
    assert_eq!(disassemble(0x00111083), "lh x1, 1(x2)");
    assert_eq!(disassemble(0x00219103), "lh x2, 2(x3)");
    assert_eq!(disassemble(0x00421203), "lh x4, 4(x4)");
    assert_eq!(disassemble(0x00829283), "lh x5, 8(x5)");
    assert_eq!(disassemble(0xfff31303), "lh x6, -1(x6)");
    assert_eq!(disassemble(0x80039383), "lh x7, -2048(x7)");
    assert_eq!(disassemble(0xffe41403), "lh x8, -2(x8)");
    assert_eq!(disassemble(0x7ff49483), "lh x9, 2047(x9)"); 
}

#[test]
fn test_disassemble_lw() {
    assert_eq!(disassemble(0x0000a003), "lw x0, 0(x1)");
    assert_eq!(disassemble(0x00112083), "lw x1, 1(x2)");
    assert_eq!(disassemble(0x0021a103), "lw x2, 2(x3)");
    assert_eq!(disassemble(0x00422203), "lw x4, 4(x4)");
    assert_eq!(disassemble(0x0082a283), "lw x5, 8(x5)");
    assert_eq!(disassemble(0xfff32303), "lw x6, -1(x6)");
    assert_eq!(disassemble(0x8003a383), "lw x7, -2048(x7)");
    assert_eq!(disassemble(0xffe42403), "lw x8, -2(x8)");
    assert_eq!(disassemble(0x7ff4a483), "lw x9, 2047(x9)");
}

#[test]
fn test_disassemble_ld() {
    assert_eq!(disassemble(0x0000b003), "ld x0, 0(x1)");
    assert_eq!(disassemble(0x00113083), "ld x1, 1(x2)");
    assert_eq!(disassemble(0x0021b103), "ld x2, 2(x3)");
    assert_eq!(disassemble(0x00423203), "ld x4, 4(x4)");
    assert_eq!(disassemble(0x0082b283), "ld x5, 8(x5)");
    assert_eq!(disassemble(0xfff33303), "ld x6, -1(x6)");
    assert_eq!(disassemble(0x8003b383), "ld x7, -2048(x7)");
    assert_eq!(disassemble(0xffe43403), "ld x8, -2(x8)");
    assert_eq!(disassemble(0x7ff4b483), "ld x9, 2047(x9)");
}

#[test]
fn test_disassemble_lhu() {
    assert_eq!(disassemble(0x0000d003), "lhu x0, 0(x1)");
    assert_eq!(disassemble(0x00115083), "lhu x1, 1(x2)");
    assert_eq!(disassemble(0x0021d103), "lhu x2, 2(x3)");
    assert_eq!(disassemble(0x00425203), "lhu x4, 4(x4)");
    assert_eq!(disassemble(0x0082d283), "lhu x5, 8(x5)");
    assert_eq!(disassemble(0xfff35303), "lhu x6, -1(x6)");
    assert_eq!(disassemble(0x8003d383), "lhu x7, -2048(x7)");
    assert_eq!(disassemble(0xffe45403), "lhu x8, -2(x8)");
    assert_eq!(disassemble(0x7ff4d483), "lhu x9, 2047(x9)");
}

#[test]
fn test_disassemble_lwu() {
    assert_eq!(disassemble(0x0000e003), "lwu x0, 0(x1)");
    assert_eq!(disassemble(0x00116083), "lwu x1, 1(x2)");
    assert_eq!(disassemble(0x0021e103), "lwu x2, 2(x3)");
    assert_eq!(disassemble(0x00426203), "lwu x4, 4(x4)");
    assert_eq!(disassemble(0x0082e283), "lwu x5, 8(x5)");
    assert_eq!(disassemble(0xfff36303), "lwu x6, -1(x6)");
    assert_eq!(disassemble(0x8003e383), "lwu x7, -2048(x7)");
    assert_eq!(disassemble(0xffe46403), "lwu x8, -2(x8)");
    assert_eq!(disassemble(0x7ff4e483), "lwu x9, 2047(x9)");
}


#[test]
fn test_disassemble_add() {
      assert_eq!(disassemble(0x00208033), "add x0, x1, x2");
      assert_eq!(disassemble(0x005201b3), "add x3, x4, x5");
      assert_eq!(disassemble(0x00838333), "add x6, x7, x8");
      assert_eq!(disassemble(0x00b504b3), "add x9, x10, x11");
      assert_eq!(disassemble(0x00e60633), "add x12, x12, x14");
      assert_eq!(disassemble(0x011807b3), "add x15, x16, x17");
      assert_eq!(disassemble(0x01498933), "add x18, x19, x20");
      assert_eq!(disassemble(0x017b0ab3), "add x21, x22, x23");
      assert_eq!(disassemble(0x01ac8c33), "add x24, x25, x26");
      assert_eq!(disassemble(0x01de0db3), "add x27, x28, x29");
      assert_eq!(disassemble(0x01ff8f33), "add x30, x31, x31");
}

#[test]
fn test_disassemble_sub() {
    assert_eq!(disassemble(0x40208033), "sub x0, x1, x2");
    assert_eq!(disassemble(0x405201b3), "sub x3, x4, x5");
    assert_eq!(disassemble(0x40838333), "sub x6, x7, x8");
    assert_eq!(disassemble(0x40b504b3), "sub x9, x10, x11");
    assert_eq!(disassemble(0x40e60633), "sub x12, x12, x14");
    assert_eq!(disassemble(0x411807b3), "sub x15, x16, x17");
    assert_eq!(disassemble(0x41498933), "sub x18, x19, x20");
    assert_eq!(disassemble(0x417b0ab3), "sub x21, x22, x23");
    assert_eq!(disassemble(0x41ac8c33), "sub x24, x25, x26");
    assert_eq!(disassemble(0x41de0db3), "sub x27, x28, x29");
    assert_eq!(disassemble(0x41ff8f33), "sub x30, x31, x31");
}

#[test]
fn test_disassemble_sll() {
    assert_eq!(disassemble(0x00209033), "sll x0, x1, x2");
    assert_eq!(disassemble(0x005211b3), "sll x3, x4, x5");
    assert_eq!(disassemble(0x00839333), "sll x6, x7, x8");
    assert_eq!(disassemble(0x00b514b3), "sll x9, x10, x11");
    assert_eq!(disassemble(0x00e61633), "sll x12, x12, x14");
    assert_eq!(disassemble(0x011817b3), "sll x15, x16, x17");
    assert_eq!(disassemble(0x01499933), "sll x18, x19, x20");
    assert_eq!(disassemble(0x017b1ab3), "sll x21, x22, x23");
    assert_eq!(disassemble(0x01ac9c33), "sll x24, x25, x26");
    assert_eq!(disassemble(0x01de1db3), "sll x27, x28, x29");
    assert_eq!(disassemble(0x01ff9f33), "sll x30, x31, x31");
}

#[test]
fn test_disassemble_slt() {
    assert_eq!(disassemble(0x0020a033), "slt x0, x1, x2");
    assert_eq!(disassemble(0x005221b3), "slt x3, x4, x5");
    assert_eq!(disassemble(0x0083a333), "slt x6, x7, x8");
    assert_eq!(disassemble(0x00b524b3), "slt x9, x10, x11");
    assert_eq!(disassemble(0x00e62633), "slt x12, x12, x14");
    assert_eq!(disassemble(0x011827b3), "slt x15, x16, x17");
    assert_eq!(disassemble(0x0149a933), "slt x18, x19, x20");
    assert_eq!(disassemble(0x017b2ab3), "slt x21, x22, x23");
    assert_eq!(disassemble(0x01acac33), "slt x24, x25, x26");
    assert_eq!(disassemble(0x01de2db3), "slt x27, x28, x29");
    assert_eq!(disassemble(0x01ffaf33), "slt x30, x31, x31");
}

#[test]
fn test_disassemble_sltu() {
    assert_eq!(disassemble(0x0020b033), "sltu x0, x1, x2");
    assert_eq!(disassemble(0x005231b3), "sltu x3, x4, x5");
    assert_eq!(disassemble(0x0083b333), "sltu x6, x7, x8");
    assert_eq!(disassemble(0x00b534b3), "sltu x9, x10, x11");
    assert_eq!(disassemble(0x00e63633), "sltu x12, x12, x14");
    assert_eq!(disassemble(0x011837b3), "sltu x15, x16, x17");
    assert_eq!(disassemble(0x0149b933), "sltu x18, x19, x20");
    assert_eq!(disassemble(0x017b3ab3), "sltu x21, x22, x23");
    assert_eq!(disassemble(0x01acbc33), "sltu x24, x25, x26");
    assert_eq!(disassemble(0x01de3db3), "sltu x27, x28, x29");
    assert_eq!(disassemble(0x01ffbf33), "sltu x30, x31, x31");
}

#[test]
fn test_disassemble_xor() {
    assert_eq!(disassemble(0x0020c033), "xor x0, x1, x2");
    assert_eq!(disassemble(0x005241b3), "xor x3, x4, x5");
    assert_eq!(disassemble(0x0083c333), "xor x6, x7, x8");
    assert_eq!(disassemble(0x00b544b3), "xor x9, x10, x11");
    assert_eq!(disassemble(0x00e64633), "xor x12, x12, x14");
    assert_eq!(disassemble(0x011847b3), "xor x15, x16, x17");
    assert_eq!(disassemble(0x0149c933), "xor x18, x19, x20");
    assert_eq!(disassemble(0x017b4ab3), "xor x21, x22, x23");
    assert_eq!(disassemble(0x01accc33), "xor x24, x25, x26");
    assert_eq!(disassemble(0x01de4db3), "xor x27, x28, x29");
    assert_eq!(disassemble(0x01ffcf33), "xor x30, x31, x31");  
}

#[test]
fn test_disassemble_srl() {
    assert_eq!(disassemble(0x0020d033), "srl x0, x1, x2");
    assert_eq!(disassemble(0x005251b3), "srl x3, x4, x5");
    assert_eq!(disassemble(0x0083d333), "srl x6, x7, x8");
    assert_eq!(disassemble(0x00b554b3), "srl x9, x10, x11");
    assert_eq!(disassemble(0x00e65633), "srl x12, x12, x14");
    assert_eq!(disassemble(0x011857b3), "srl x15, x16, x17");
    assert_eq!(disassemble(0x0149d933), "srl x18, x19, x20");
    assert_eq!(disassemble(0x017b5ab3), "srl x21, x22, x23");
    assert_eq!(disassemble(0x01acdc33), "srl x24, x25, x26");
    assert_eq!(disassemble(0x01de5db3), "srl x27, x28, x29");
    assert_eq!(disassemble(0x01ffdf33), "srl x30, x31, x31");
}

#[test]
fn test_disassemble_or() {
    assert_eq!(disassemble(0x0020e033), "or x0, x1, x2");
    assert_eq!(disassemble(0x005261b3), "or x3, x4, x5");
    assert_eq!(disassemble(0x0083e333), "or x6, x7, x8");
    assert_eq!(disassemble(0x00b564b3), "or x9, x10, x11");
    assert_eq!(disassemble(0x00e66633), "or x12, x12, x14");
    assert_eq!(disassemble(0x011867b3), "or x15, x16, x17");
    assert_eq!(disassemble(0x0149e933), "or x18, x19, x20");
    assert_eq!(disassemble(0x017b6ab3), "or x21, x22, x23");
    assert_eq!(disassemble(0x01acec33), "or x24, x25, x26");
    assert_eq!(disassemble(0x01de6db3), "or x27, x28, x29");
    assert_eq!(disassemble(0x01ffef33), "or x30, x31, x31");
}

#[test]
fn test_disassemble_sra() {
    assert_eq!(disassemble(0x4020d033), "sra x0, x1, x2");
    assert_eq!(disassemble(0x405251b3), "sra x3, x4, x5");
    assert_eq!(disassemble(0x4083d333), "sra x6, x7, x8");
    assert_eq!(disassemble(0x40b554b3), "sra x9, x10, x11");
    assert_eq!(disassemble(0x40e65633), "sra x12, x12, x14");
    assert_eq!(disassemble(0x411857b3), "sra x15, x16, x17");
    assert_eq!(disassemble(0x4149d933), "sra x18, x19, x20");
    assert_eq!(disassemble(0x417b5ab3), "sra x21, x22, x23");
    assert_eq!(disassemble(0x41acdc33), "sra x24, x25, x26");
    assert_eq!(disassemble(0x41de5db3), "sra x27, x28, x29");
    assert_eq!(disassemble(0x41ffdf33), "sra x30, x31, x31");
}

#[test]
fn test_disassemble_sb() {
      assert_eq!(disassemble(0x00008023), "sb x0, 0(x1)");
      assert_eq!(disassemble(0xfe218fa3), "sb x2, -1(x3)");
      assert_eq!(disassemble(0x7e428fa3), "sb x4, 2047(x5)");
      assert_eq!(disassemble(0x80530023), "sb x5, -2048(x6)");
      assert_eq!(disassemble(0x00638123), "sb x6, 2(x7)");
      assert_eq!(disassemble(0x00848223), "sb x8, 4(x9)");
      assert_eq!(disassemble(0x00a58423), "sb x10, 8(x11)");
      assert_eq!(disassemble(0x00c68823), "sb x12, 16(x13)");
      assert_eq!(disassemble(0x02e78023), "sb x14, 32(x15)");
      assert_eq!(disassemble(0x05088023), "sb x16, 64(x17)");
}

#[test]
fn test_disassemble_sw() {
    assert_eq!(disassemble(0x0000a023), "sw x0, 0(x1)");
    assert_eq!(disassemble(0xfe21afa3), "sw x2, -1(x3)");
    assert_eq!(disassemble(0x7e42afa3), "sw x4, 2047(x5)");
    assert_eq!(disassemble(0x80532023), "sw x5, -2048(x6)");
    assert_eq!(disassemble(0x0063a123), "sw x6, 2(x7)");
    assert_eq!(disassemble(0x0084a223), "sw x8, 4(x9)");
    assert_eq!(disassemble(0x00a5a423), "sw x10, 8(x11)");
    assert_eq!(disassemble(0x00c6a823), "sw x12, 16(x13)");
    assert_eq!(disassemble(0x02e7a023), "sw x14, 32(x15)");
    assert_eq!(disassemble(0x0508a023), "sw x16, 64(x17)");
}


#[test]
fn test_disassemble_sd() {
    assert_eq!(disassemble(0x0000b023), "sd x0, 0(x1)");
    assert_eq!(disassemble(0xfe21bfa3), "sd x2, -1(x3)");
    assert_eq!(disassemble(0x7e42bfa3), "sd x4, 2047(x5)");
    assert_eq!(disassemble(0x80533023), "sd x5, -2048(x6)");
    assert_eq!(disassemble(0x0063b123), "sd x6, 2(x7)");
    assert_eq!(disassemble(0x0084b223), "sd x8, 4(x9)");
    assert_eq!(disassemble(0x00a5b423), "sd x10, 8(x11)");
    assert_eq!(disassemble(0x00c6b823), "sd x12, 16(x13)");
    assert_eq!(disassemble(0x02e7b023), "sd x14, 32(x15)");
    assert_eq!(disassemble(0x0508b023), "sd x16, 64(x17)");
}

#[test]
fn test_disassemble_beq() {
    assert_eq!(disassemble(0x00000063), "beq x0, x0, 0"); 
    assert_eq!(disassemble(0xfe208ee3), "beq x1, x2, -4"); 
    assert_eq!(disassemble(0xfe418ce3), "beq x3, x4, -8"); 
    assert_eq!(disassemble(0xfe628ae3), "beq x5, x6, -12"); 
    assert_eq!(disassemble(0x00838c63), "beq x7, x8, 24"); 
    assert_eq!(disassemble(0x00a48a63), "beq x9, x10, 20"); 
    assert_eq!(disassemble(0x00c58863), "beq x11, x12, 16"); 
    assert_eq!(disassemble(0x00e68663), "beq x13, x14, 12"); 
    assert_eq!(disassemble(0x01078463), "beq x15, x16, 8"); 
    assert_eq!(disassemble(0x01288263), "beq x17, x18, 4"); 
}

#[test]
fn test_disassemble_bne() {
    assert_eq!(disassemble(0x00001063), "bne x0, x0, 0");
    assert_eq!(disassemble(0xfe209ee3), "bne x1, x2, -4");
    assert_eq!(disassemble(0xfe419ce3), "bne x3, x4, -8");
    assert_eq!(disassemble(0xfe629ae3), "bne x5, x6, -12");
    assert_eq!(disassemble(0x00839c63), "bne x7, x8, 24");
    assert_eq!(disassemble(0x00a49a63), "bne x9, x10, 20");
    assert_eq!(disassemble(0x00c59863), "bne x11, x12, 16");
    assert_eq!(disassemble(0x00e69663), "bne x13, x14, 12");
    assert_eq!(disassemble(0x01079463), "bne x15, x16, 8");
    assert_eq!(disassemble(0x01289263), "bne x17, x18, 4");
}

#[test]
fn test_disassemble_blt() {
    assert_eq!(disassemble(0x00004063), "blt x0, x0, 0");
    assert_eq!(disassemble(0xfe20cee3), "blt x1, x2, -4");
    assert_eq!(disassemble(0xfe41cce3), "blt x3, x4, -8");
    assert_eq!(disassemble(0xfe62cae3), "blt x5, x6, -12");
    assert_eq!(disassemble(0x0083cc63), "blt x7, x8, 24");
    assert_eq!(disassemble(0x00a4ca63), "blt x9, x10, 20");
    assert_eq!(disassemble(0x00c5c863), "blt x11, x12, 16");
    assert_eq!(disassemble(0x00e6c663), "blt x13, x14, 12");
    assert_eq!(disassemble(0x0107c463), "blt x15, x16, 8");
    assert_eq!(disassemble(0x0128c263), "blt x17, x18, 4"); 
}

#[test]
fn test_disassemble_bge() {
    assert_eq!(disassemble(0x00005063), "bge x0, x0, 0");
    assert_eq!(disassemble(0xfe20dee3), "bge x1, x2, -4");
    assert_eq!(disassemble(0xfe41dce3), "bge x3, x4, -8");
    assert_eq!(disassemble(0xfe62dae3), "bge x5, x6, -12");
    assert_eq!(disassemble(0x0083dc63), "bge x7, x8, 24");
    assert_eq!(disassemble(0x00a4da63), "bge x9, x10, 20");
    assert_eq!(disassemble(0x00c5d863), "bge x11, x12, 16");
    assert_eq!(disassemble(0x00e6d663), "bge x13, x14, 12");
    assert_eq!(disassemble(0x0107d463), "bge x15, x16, 8");
    assert_eq!(disassemble(0x0128d263), "bge x17, x18, 4"); 
}

#[test]
fn test_disassemble_bltu() {
    assert_eq!(disassemble(0x00006063), "bltu x0, x0, 0");
    assert_eq!(disassemble(0xfe20eee3), "bltu x1, x2, -4");
    assert_eq!(disassemble(0xfe41ece3), "bltu x3, x4, -8");
    assert_eq!(disassemble(0xfe62eae3), "bltu x5, x6, -12");
    assert_eq!(disassemble(0x0083ec63), "bltu x7, x8, 24");
    assert_eq!(disassemble(0x00a4ea63), "bltu x9, x10, 20");
    assert_eq!(disassemble(0x00c5e863), "bltu x11, x12, 16");
    assert_eq!(disassemble(0x00e6e663), "bltu x13, x14, 12");
    assert_eq!(disassemble(0x0107e463), "bltu x15, x16, 8");
    assert_eq!(disassemble(0x0128e263), "bltu x17, x18, 4"); 
}

#[test]
fn test_disassemble_bgeu() {
    assert_eq!(disassemble(0x00007063), "bgeu x0, x0, 0");
    assert_eq!(disassemble(0xfe20fee3), "bgeu x1, x2, -4");
    assert_eq!(disassemble(0xfe41fce3), "bgeu x3, x4, -8");
    assert_eq!(disassemble(0xfe62fae3), "bgeu x5, x6, -12");
    assert_eq!(disassemble(0x0083fc63), "bgeu x7, x8, 24");
    assert_eq!(disassemble(0x00a4fa63), "bgeu x9, x10, 20");
    assert_eq!(disassemble(0x00c5f863), "bgeu x11, x12, 16");
    assert_eq!(disassemble(0x00e6f663), "bgeu x13, x14, 12");
    assert_eq!(disassemble(0x0107f463), "bgeu x15, x16, 8");
    assert_eq!(disassemble(0x0128f263), "bgeu x17, x18, 4"); 
}

#[test]
fn test_disassemble_lui() {
    assert_eq!(disassemble(0x00000037), "lui x0, 0x00000");
    assert_eq!(disassemble(0x000010b7), "lui x1, 0x00001");
    assert_eq!(disassemble(0x00020137), "lui x2, 0x00020");
    assert_eq!(disassemble(0x004001b7), "lui x3, 0x00400");
    assert_eq!(disassemble(0x08000237), "lui x4, 0x08000");
    assert_eq!(disassemble(0x7ffff2b7), "lui x5, 0x7FFFF");
    assert_eq!(disassemble(0x80000337), "lui x6, 0x80000");
    assert_eq!(disassemble(0xfffff3b7), "lui x7, 0xFFFFF");
}

#[test]
fn test_disassemble_auipc() {
    assert_eq!(disassemble(0x00000017), "auipc x0, 0x00000");
    assert_eq!(disassemble(0x00001097), "auipc x1, 0x00001");
    assert_eq!(disassemble(0x00020117), "auipc x2, 0x00020");
    assert_eq!(disassemble(0x00400197), "auipc x3, 0x00400");
    assert_eq!(disassemble(0x08000217), "auipc x4, 0x08000");
    assert_eq!(disassemble(0x7ffff297), "auipc x5, 0x7FFFF");
    assert_eq!(disassemble(0x80000317), "auipc x6, 0x80000");
    assert_eq!(disassemble(0xfffff397), "auipc x7, 0xFFFFF");
}
