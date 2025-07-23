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
const FIELD_3B: u32 = 0x07;  //-- Campo de 3 bits de ancho
const FIELD_4B: u32 = 0x0F;  //-- Campo de 4 bits
const FIELD_5B: u32 = 0x1F;  //-- Campo de 5 bits
const FIELD_6B: u32 = 0x3F;  //-- Campo de 6 bits
const FIELD_7B: u32 = 0x7F;  //-- Campo de 7 bits
const FIELD_8B: u32 = 0xFF;  //-- Campo de 8 bits
const FIELD_10B: u32 = 0x3FF; //-- Campo de 10 bits
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
const OFFSET8_MASK: u32 = FIELD_8B << OFFSET8_POS; 
const OFFSET10_MASK: u32 = FIELD_10B << OFFSET10_POS;
const OFFSET6_MASK: u32 = FIELD_6B << OFFSET6_POS;
const OFFSET5_MASK: u32 = FIELD_5B << OFFSET5_POS;
const OFFSET4_MASK: u32 = FIELD_4B << OFFSET4_POS;
const OFFSET1_MASK: u32 = FIELD_1B << OFFSET1_POS;

//────────────────────────────────────────────────
//  DEFINICION DE LOS OPCODES
//────────────────────────────────────────────────
enum OpcodeRV {
  //-- Instrucciones aritméticas tipo I
  TipoIArith = 0x13,   //-- Ex: ADDI: addi rd, rs1, imm12

  //-- Instrucciones de carga (tipo I)
  TipoILoad = 0x03,    //-- Ex: LW: lw rd, imm12(rs1)
}

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
//  Instruccion tipo-J: jal
//────────────────────────────
const OPCODE_J_JAL: u32 = 0b_11011_11; //--0x6F
//  Instruccion tipo-J: jalr
//────────────────────────────
const OPCODE_J_JALR: u32 = 0b_11001_11; //--0x67
//  Instruccion tipo ecall/ebreak
//────────────────────────────
const OPCODE_ECALL_EBREAK: u32 = 0b_11100_11; //--0x73


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



fn is_type_i(opcode: u32) -> bool {
//────────────────────────────────────────────────
// Entrada: opcode RISC-V
// Salida: true si es una instrucción tipo I
//────────────────────────────────────────────────
    //-- Las instrucciones de tipo I son bien las de tipo
    //-- aritmético (ADDI, ANDI, ORI,...) o las de LOAD (LW, LH, LB,...)
    if opcode == OpcodeRV::TipoIArith as u32 || 
       opcode == OpcodeRV::TipoILoad as u32 {
        true
    } else {
        false
    }
}

fn is_type_i_arith(opcode: u32) -> bool {
    if opcode == OpcodeRV::TipoIArith as u32 {
        true
    }
    else {
        false
    }
}

fn is_type_i_load(opcode: u32) -> bool {
    if opcode == OpcodeRV::TipoILoad as u32 {
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

fn is_type_j_jal(opcode: u32) -> bool {
    if opcode == OPCODE_J_JAL {
      true
    }
    else {
      false
    }
}

fn is_type_j_jalr(opcode: u32) -> bool {
    if opcode == OPCODE_J_JALR {
      true
    }
    else {
      false
    }
}

fn is_ecall_ebreak(opcode: u32) -> bool {
    if opcode == OPCODE_ECALL_EBREAK {
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
    } else if is_type_j_jal(opcode) {
        let rd: u32 = get_rd(inst);
        let offset: i32 = get_offset_jal(inst);
        format!("jal x{}, {}", rd, offset)
    }
    else if is_type_j_jalr(opcode) {
        //-- Instrucción jalr
        let rd: u32 = get_rd(inst);
        let rs1: u32 = get_rs1(inst);
        let offset: i32 = get_imm12(inst);

        format!("jalr x{}, {}(x{})", rd, offset, rs1)
    } else if is_ecall_ebreak(opcode) {
        let imm: i32 = get_imm12(inst);
        if imm == 0 {
            format!("ecall")
        } else if imm == 1 {
            format!("ebreak")
        } else {
            format!("DESCONOCIDA")
        }

    } else
    {
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
  assert!(is_type_i(OpcodeRV::TipoIArith as u32));
  assert!(is_type_i(OpcodeRV::TipoILoad as u32));
  assert!(is_type_i(0x13));  //-- ADDI
  assert!(is_type_i(0x03));  //-- LW

  //-- Tipos incorrectos
  assert!(!is_type_i(0x00));
  assert!(!is_type_i(0x01));
  assert!(!is_type_i(0x02));
  assert!(!is_type_i(0x04));
  assert!(!is_type_i(0xFF));
}



