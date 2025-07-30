use crate::{mcode, regs::Reg, opcoderv::OpcodeRV};

//────────────────────────────────────────────────
//  POSICIONES Bits aislados
//────────────────────────────────────────────────
const BIT5: u32 = 1 << 5;
const BIT10: u32 = 1 << 10;

//────────────────────────────────────────────────
//  Instrucciones del RISC-V
//────────────────────────────────────────────────    
pub enum InstructionRV {

    //──────────────────────────────────
    //  Instrucciones aritméticas tipo I
    //──────────────────────────────────
    Addi {rd: Reg, rs1: Reg, imm: i32},  //-- addi rd, rs1, imm12
    Slli {rd: Reg, rs1: Reg, imm: i32},  //-- slli x1, x2, 1
    Slti {rd: Reg, rs1: Reg, imm: i32},  //-- slti rd, rs1, imm12
    Sltiu {rd: Reg, rs1: Reg, imm: i32}, //-- sltiu rd, rs1, imm12
    Xori {rd: Reg, rs1: Reg, imm: i32},  //-- xori rd, rs1, imm12
    Srli {rd: Reg, rs1: Reg, imm: i32},  //-- srli rd, rs1, imm12
    Srai {rd: Reg, rs1: Reg, imm: i32},  //-- srai rd, rs1, imm12
    Ori {rd: Reg, rs1: Reg, imm: i32},   //-- ori rd, rs1, imm12
    Andi {rd: Reg, rs1: Reg, imm: i32},  //-- andi rd, rs1, imm12

    //──────────────────────────────────
    //  Instrucciones tipo I: LOAD
    //──────────────────────────────────
    Lb {rd: Reg, offs: i32, rs1: Reg},  //-- lb rd, off(rs1)
    Lh {rd: Reg, offs: i32, rs1: Reg},  //-- lh rd, off(rs1)
    Lw {rd: Reg, offs: i32, rs1: Reg},  //-- lw rd, off(rs1)
    Ld {rd: Reg, offs: i32, rs1: Reg},  //-- ld rd, off(rs1)
    Lbu {rd: Reg, offs: i32, rs1: Reg}, //-- lbu rd, off(rs1)
    Lhu {rd: Reg, offs: i32, rs1: Reg}, //-- lhu rd, off(rs1)
    Lwu {rd: Reg, offs: i32, rs1: Reg}, //-- lwu rd, off(rs1)
    
    //──────────────────────────────────
    //  Instrucciones tipo R
    //──────────────────────────────────
    Add {rd: Reg, rs1: Reg, rs2: Reg}, //-- add rd, rs1, rs2

    Unknown, //-- Instrucción desconocida
}


impl InstructionRV {
    pub fn from_mcode(mcode: u32) -> Self {

        let mcode = mcode::MCode::new(mcode);
        let func3 = mcode.func3();
        let func7 = mcode.func7();

        match mcode.opcode() {
            OpcodeRV::TipoIArith => {

                let imm = mcode.imm12();
                //-- Caso especial: srli y srai tienen el mismo codigo func3
                //-- Se diferenencias por el bit 30 del opcode (bit 10 del valor imm)
                let bit_srali:u32 = imm as u32 >> 10; 

                //-- Gestionar caso especial
                if (bit_srali==1) && (func3==0b101) {

                    //-- Caso especial: srai
                    //-- El 10 de imm está a 1 (en caso de srai)
                    //-- Este bit hay que ponerlo a 0
                    let imm2:i32 = (imm as u32 & !BIT10) as i32;
                   
                    return Self::Srai {
                        rd: mcode.rd(),
                        rs1: mcode.rs1(),
                        imm: imm2
                    };
                }
                //-- CASO GENERAL
                else {

                //-- Según el campo func3, determinar la instrucción
                    match func3 {
                        0b_000 => Self::Addi {
                                    rd: mcode.rd(),
                                    rs1: mcode.rs1(),
                                    imm: mcode.imm12()},
                        0b_001 => Self::Slli { 
                                    rd: mcode.rd(), 
                                    rs1: mcode.rs1(), 
                                    imm: mcode.imm12() 
                                  },
                        0b_010 => Self::Slti { 
                                    rd: mcode.rd(), 
                                    rs1: mcode.rs1(), 
                                    imm: mcode.imm12() 
                                  },
                        0b_011 => Self::Sltiu { 
                                    rd: mcode.rd(), 
                                    rs1: mcode.rs1(), 
                                    imm: mcode.imm12() 
                                  },
                        0b_100 => Self::Xori { 
                                    rd: mcode.rd(), 
                                    rs1: mcode.rs1(), 
                                    imm: mcode.imm12() 
                                  },
                        0b_101 => Self::Srli { 
                                    rd: mcode.rd(), 
                                    rs1: mcode.rs1(), 
                                    imm: mcode.imm12() 
                                  },
                        0b_110 => Self::Ori {
                                    rd: mcode.rd(),
                                    rs1: mcode.rs1(),
                                    imm: mcode.imm12()
                                  },
                        0b_111 => Self::Andi {
                                    rd: mcode.rd(),
                                    rs1: mcode.rs1(),
                                    imm: mcode.imm12()
                                  },
                        _ => Self::Unknown
                    }
                }
            },
            OpcodeRV::TipoILoad => {
                match func3 {
                    0b_000 => 
                        Self::Lb { 
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    0b_001 => 
                        Self::Lh {
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    0b_010 => 
                        Self::Lw {
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    0b_011 => 
                        Self::Ld {
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    0b_100 => 
                        Self::Lbu {
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    0b_101 => 
                        Self::Lhu {
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    0b_110 => 
                        Self::Lwu {
                            rd: mcode.rd(), 
                            offs: mcode.imm12(), 
                            rs1: mcode.rs1() 
                        },
                    _ => Self::Unknown,
                }
            },
            OpcodeRV::TipoR => {
                //-- Obtener el bit 5 de func7
                let especial: u32 = (func7 & BIT5) >> 5;

                if especial==1 {
                    //-- Instrucciones sub, sra
                    Self::Unknown
                } else {
                    //-- Resto de instrucciones tipo R
                    match func3 {
                        0b_000 => 
                            Self::Add { 
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        _ =>  Self::Unknown  
                    }
                }

            },
            _ => Self::Unknown,
        }    
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Addi {rd, rs1, imm} => {
                format!("addi {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Slli {rd, rs1, imm} => {
                format!("slli {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Slti {rd, rs1, imm} => {
                format!("slti {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Sltiu {rd, rs1, imm} => {
                format!("sltiu {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Xori {rd, rs1, imm} => {
                format!("xori {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Srli {rd, rs1, imm} => {
                format!("srli {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Ori { rd, rs1, imm } => {
                format!("ori {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Andi { rd, rs1, imm } => {
                format!("andi {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Srai {rd, rs1, imm} => {
                format!("srai {}, {}, {}", rd.to_str(), rs1.to_str(), imm)
            },
            Self::Lb { rd, offs, rs1, } => {
                format!("lb {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lh { rd, offs, rs1, } => {
                format!("lh {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lw { rd, offs, rs1, } => {
                format!("lw {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Ld { rd, offs, rs1, } => {
                format!("ld {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lbu { rd, offs, rs1, } => {
                format!("lbu {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lhu { rd, offs, rs1, } => {
                format!("lhu {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lwu { rd, offs, rs1, } => {
                format!("lwu {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Add { rd, rs1, rs2} => {
                format!("add {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            }
            Self::Unknown => {
                "Unknown Instruction".to_string()
            },
        }
    }
}

//────────────────────────────────────────────────
//  PRUEBAS DEL TIPO InstructionRV
//────────────────────────────────────────────────

#[test]
fn test_instructions_addi() {

    assert_eq!(
      InstructionRV::Addi { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
      "addi x0, x0, 0");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }.to_string(),
      "addi x1, x0, 1");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X2, rs1: Reg::X0, imm: 2 }.to_string(),
      "addi x2, x0, 2");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: -1 }.to_string(),
      "addi x3, x0, -1");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X4, rs1: Reg::X0, imm: 2047 }.to_string(),
      "addi x4, x0, 2047");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X31, rs1: Reg::X1, imm: 3 }.to_string(),
      "addi x31, x1, 3");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X8, rs1: Reg::X2, imm: 4 }.to_string(),
      "addi x8, x2, 4");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X16, rs1: Reg::X4, imm: 8 }.to_string(),
      "addi x16, x4, 8");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: 16 }.to_string(),
      "addi x17, x8, 16");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: -16 }.to_string(),
      "addi x17, x8, -16");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: -2048 }.to_string(),
      "addi x17, x8, -2048");
    assert_eq!(
      InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 170 }.to_string(),
      "addi x1, x0, 170");
}

#[test]
fn test_instructions_slli() {

    assert_eq!(
      InstructionRV::Slli { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
      "slli x0, x0, 0");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
      "slli x1, x2, 1");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
      "slli x31, x1, 2");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
      "slli x30, x2, 4");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
      "slli x29, x3, 8");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
      "slli x28, x4, 16");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
      "slli x27, x5, 17");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
      "slli x26, x6, 30");
    assert_eq!(
      InstructionRV::Slli { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
      "slli x25, x7, 31");

}


#[test]
fn test_instructions_slti() {
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "slti x0, x0, 0");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "slti x1, x2, 1");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
        "slti x31, x1, 2");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
        "slti x30, x2, 4");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
        "slti x29, x3, 8");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
        "slti x28, x4, 16");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
        "slti x27, x5, 17");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
        "slti x26, x6, 30");
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
        "slti x25, x7, 31");

}

#[test]
fn test_instructions_sltiu() {
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "sltiu x0, x0, 0");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "sltiu x1, x2, 1");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
        "sltiu x31, x1, 2");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
        "sltiu x30, x2, 4");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
        "sltiu x29, x3, 8");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
        "sltiu x28, x4, 16");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
        "sltiu x27, x5, 17");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
        "sltiu x26, x6, 30");
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
        "sltiu x25, x7, 31");
    
}

#[test]
fn test_instructions_xori() {
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "xori x0, x0, 0");
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "xori x1, x2, 1");
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
        "xori x31, x1, 2"); 
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
        "xori x30, x2, 4");
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
        "xori x29, x3, 8");
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
        "xori x28, x4, 16");        
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
        "xori x27, x5, 17");
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
        "xori x26, x6, 30");
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
        "xori x25, x7, 31");

}


#[test]
fn test_instructions_srli() {
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "srli x0, x0, 0");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "srli x1, x2, 1");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
        "srli x31, x1, 2");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
        "srli x30, x2, 4");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
        "srli x29, x3, 8");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
        "srli x28, x4, 16");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
        "srli x27, x5, 17");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
        "srli x26, x6, 30");
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
        "srli x25, x7, 31");
}    
    

#[test]
fn test_instructions_andi() {
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "andi x0, x0, 0");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "andi x1, x2, 1");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
        "andi x31, x1, 2");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
        "andi x30, x2, 4");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
        "andi x29, x3, 8");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
        "andi x28, x4, 16");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
        "andi x27, x5, 17");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
        "andi x26, x6, 30");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
        "andi x25, x7, 31");

}


#[test]
fn test_instructions_srai() {
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "srai x0, x0, 0");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "srai x1, x2, 1");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_string(),
        "srai x31, x1, 2");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_string(),
        "srai x30, x2, 4");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_string(),
        "srai x29, x3, 8");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_string(),
        "srai x28, x4, 16");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_string(),
        "srai x27, x5, 17");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_string(),
        "srai x26, x6, 30");
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_string(),
        "srai x25, x7, 31");

}


#[test]
fn test_instruction_lb() {
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
        "lb x0, 0(x1)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X1, offs: 1, rs1: Reg::X2 }.to_string(),
         "lb x1, 1(x2)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X2, offs: 2, rs1: Reg::X3 }.to_string(),
         "lb x2, 2(x3)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X4, offs: 4, rs1: Reg::X4 }.to_string(),
         "lb x4, 4(x4)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X5, offs: 8, rs1: Reg::X5 }.to_string(),
         "lb x5, 8(x5)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X6, offs: -1, rs1: Reg::X6 }.to_string(),
         "lb x6, -1(x6)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X7, offs: -2048, rs1: Reg::X7 }.to_string(),
         "lb x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X8, offs: -2, rs1: Reg::X8 }.to_string(),
         "lb x8, -2(x8)");
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X9, offs: 2047, rs1: Reg::X9 }.to_string(),
         "lb x9, 2047(x9)");

}


#[test]
fn test_instruction_lh() {
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "lh x0, 0(x1)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_string(), 
        "lh x1, 1(x2)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_string(), 
        "lh x2, 2(x3)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_string(), 
        "lh x4, 4(x4)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_string(), 
        "lh x5, 8(x5)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_string(), 
        "lh x6, -1(x6)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_string(), 
        "lh x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_string(), 
        "lh x8, -2(x8)");
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_string(), 
        "lh x9, 2047(x9)"); 
}

#[test]
fn test_instruction_lw() {
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "lw x0, 0(x1)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_string(), 
        "lw x1, 1(x2)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_string(), 
        "lw x2, 2(x3)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_string(), 
        "lw x4, 4(x4)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_string(), 
        "lw x5, 8(x5)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_string(), 
        "lw x6, -1(x6)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_string(), 
        "lw x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_string(), 
        "lw x8, -2(x8)");
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_string(), 
        "lw x9, 2047(x9)");
}

#[test]
fn test_instruction_ld() {
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "ld x0, 0(x1)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_string(), 
        "ld x1, 1(x2)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_string(), 
        "ld x2, 2(x3)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_string(), 
        "ld x4, 4(x4)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_string(), 
        "ld x5, 8(x5)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_string(), 
        "ld x6, -1(x6)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_string(), 
        "ld x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_string(), 
        "ld x8, -2(x8)");
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_string(), 
        "ld x9, 2047(x9)");
}

#[test]
fn test_instruction_lbu() {
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "lbu x0, 0(x1)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_string(), 
        "lbu x1, 1(x2)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_string(), 
        "lbu x2, 2(x3)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_string(), 
        "lbu x4, 4(x4)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_string(), 
        "lbu x5, 8(x5)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_string(), 
        "lbu x6, -1(x6)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_string(), 
        "lbu x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_string(), 
        "lbu x8, -2(x8)");
    assert_eq!(
        InstructionRV::Lbu{rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_string(), 
        "lbu x9, 2047(x9)");
}

#[test]
fn test_instruction_lhu() {
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "lhu x0, 0(x1)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_string(), 
        "lhu x1, 1(x2)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_string(), 
        "lhu x2, 2(x3)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_string(), 
        "lhu x4, 4(x4)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_string(), 
        "lhu x5, 8(x5)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_string(), 
        "lhu x6, -1(x6)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_string(), 
        "lhu x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_string(), 
        "lhu x8, -2(x8)");
    assert_eq!(
        InstructionRV::Lhu{rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_string(), 
        "lhu x9, 2047(x9)");
}


#[test]
fn test_instruction_lwu() {
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "lwu x0, 0(x1)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_string(), 
        "lwu x1, 1(x2)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_string(), 
        "lwu x2, 2(x3)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_string(), 
        "lwu x4, 4(x4)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_string(), 
        "lwu x5, 8(x5)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_string(), 
        "lwu x6, -1(x6)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_string(), 
        "lwu x7, -2048(x7)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_string(), 
        "lwu x8, -2(x8)");
    assert_eq!(
        InstructionRV::Lwu{rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_string(), 
        "lwu x9, 2047(x9)");
}

#[test]
fn test_instruction_add() {
    assert_eq!(
        InstructionRV::Add{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "add x0, x1, x2");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "add x3, x4, x5");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "add x6, x7, x8");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "add x9, x10, x11");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "add x12, x12, x14");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "add x15, x16, x17");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "add x18, x19, x20");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "add x21, x22, x23");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "add x24, x25, x26");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "add x27, x28, x29");
    assert_eq!(
        InstructionRV::Add{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "add x30, x31, x31");
}


#[test]
fn test_instruction_sub() {
    //assert_eq!(disassemble(0x40208033), "sub x0, x1, x2");
    //assert_eq!(disassemble(0x405201b3), "sub x3, x4, x5");
    //assert_eq!(disassemble(0x40838333), "sub x6, x7, x8");
    //assert_eq!(disassemble(0x40b504b3), "sub x9, x10, x11");
    //assert_eq!(disassemble(0x40e60633), "sub x12, x12, x14");
    //assert_eq!(disassemble(0x411807b3), "sub x15, x16, x17");
    //assert_eq!(disassemble(0x41498933), "sub x18, x19, x20");
    //assert_eq!(disassemble(0x417b0ab3), "sub x21, x22, x23");
    //assert_eq!(disassemble(0x41ac8c33), "sub x24, x25, x26");
    //assert_eq!(disassemble(0x41de0db3), "sub x27, x28, x29");
    //assert_eq!(disassemble(0x41ff8f33), "sub x30, x31, x31");
}


//────────────────────────────────────────────────
//  PRUEBAS DEL CODIGO MAQUINA
//────────────────────────────────────────────────


#[test]
fn test_mcode_addi() {
    //────────────────────────────────────────────────
    //  Test de la instrucción ADDI
    //────────────────────────────────────────────────
    assert_eq!(
        InstructionRV::from_mcode(0x00000013).to_string(),
        "addi x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00100093).to_string(),
        "addi x1, x0, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x00200113).to_string(),
        "addi x2, x0, 2");
    assert_eq!(
        InstructionRV::from_mcode(0xfff00193).to_string(),
        "addi x3, x0, -1");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff00213).to_string(),
        "addi x4, x0, 2047");
    assert_eq!(
        InstructionRV::from_mcode(0x00308f93).to_string(),
        "addi x31, x1, 3"); 
    assert_eq!(
        InstructionRV::from_mcode(0x00410413).to_string(),
        "addi x8, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x00820813).to_string(),
        "addi x16, x4, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01040893).to_string(),
        "addi x17, x8, 16");
    assert_eq!(
        InstructionRV::from_mcode(0xff040893).to_string(),
        "addi x17, x8, -16");
    assert_eq!(
        InstructionRV::from_mcode(0x80040893).to_string(),
        "addi x17, x8, -2048");
    assert_eq!(
        InstructionRV::from_mcode(0x0aa00093).to_string(),
        "addi x1, x0, 170");  
}   


#[test]
fn test_mcode_slli() {

    assert_eq!(
        InstructionRV::from_mcode(0x00111093).to_string(), 
        "slli x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x00001013).to_string(),
        "slli x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00209f93).to_string(),
        "slli x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00411f13).to_string(),
        "slli x30, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x00819e93).to_string(),
        "slli x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01021e13).to_string(),
        "slli x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x01129d93).to_string(),
        "slli x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e31d13).to_string(),
        "slli x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f39c93).to_string(),
        "slli x25, x7, 31");
}

#[test]
fn test_mcode_slti() {
    assert_eq!(
        InstructionRV::from_mcode(0x00002013).to_string(),
        "slti x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00112093).to_string(),
        "slti x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x0020af93).to_string(),
        "slti x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00412f13).to_string(),
        "slti x30, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x0081ae93).to_string(),
        "slti x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01022e13).to_string(),
        "slti x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x0112ad93).to_string(),
        "slti x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e32d13).to_string(),
        "slti x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f3ac93).to_string(),
        "slti x25, x7, 31");

}

#[test]
fn test_mcode_sltiu() {
    assert_eq!(
        InstructionRV::from_mcode(0x00003013).to_string(),
        "sltiu x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00113093).to_string(),
        "sltiu x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x0020bf93).to_string(),
        "sltiu x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00413f13).to_string(),
        "sltiu x30, x2, 4");
    assert_eq!(             
        InstructionRV::from_mcode(0x0081be93).to_string(),
        "sltiu x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01023e13).to_string(),
        "sltiu x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x0112bd93).to_string(),
        "sltiu x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e33d13).to_string(),
        "sltiu x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f3bc93).to_string(),
        "sltiu x25, x7, 31");

}


#[test]
fn test_mcode_xori() {

    assert_eq!(
        InstructionRV::from_mcode(0x00004013).to_string(),
        "xori x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00114093).to_string(),
        "xori x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x0020cf93).to_string(),
        "xori x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00414f13).to_string(),
        "xori x30, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x0081ce93).to_string(),
        "xori x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01024e13).to_string(),
        "xori x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x0112cd93).to_string(),
        "xori x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e34d13).to_string(),
        "xori x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f3cc93).to_string(),
        "xori x25, x7, 31");

}

#[test]
fn test_mcode_srli() {
    assert_eq!(
        InstructionRV::from_mcode(0x00005013).to_string(),
        "srli x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00115093).to_string(),
        "srli x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x0020df93).to_string(),
        "srli x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00415f13).to_string(),
        "srli x30, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x0081de93).to_string(),
        "srli x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01025e13).to_string(),
        "srli x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x0112dd93).to_string(),
        "srli x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e35d13).to_string(),
        "srli x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f3dc93).to_string(),
        "srli x25, x7, 31");

}

#[test]
fn test_mcode_andi() {
    assert_eq!(
        InstructionRV::from_mcode(0x00007013).to_string(),
        "andi x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00117093).to_string(),
        "andi x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x0020ff93).to_string(),
        "andi x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00417f13).to_string(),
        "andi x30, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x0081fe93).to_string(),
        "andi x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01027e13).to_string(),
        "andi x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x0112fd93).to_string(),
        "andi x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e37d13).to_string(),
        "andi x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f3fc93).to_string(),
        "andi x25, x7, 31");

}

#[test]
fn test_mcode_srai() {
    assert_eq!(
        InstructionRV::from_mcode(0x40005013).to_string(), 
        "srai x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x40115093).to_string(),
        "srai x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x4020df93).to_string(),
        "srai x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x40415f13).to_string(),
        "srai x30, x2, 4"); 
    assert_eq!(
        InstructionRV::from_mcode(0x4081de93).to_string(),
        "srai x29, x3, 8"); 
    assert_eq!(
        InstructionRV::from_mcode(0x41025e13).to_string(),
        "srai x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x4112dd93).to_string(),
        "srai x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x41e35d13).to_string(),
        "srai x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x41f3dc93).to_string(),
        "srai x25, x7, 31");
   
}


#[test]
fn test_mcode_lb() {
    assert_eq!(
        InstructionRV::from_mcode(0x00008003).to_string(),
        "lb x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00110083).to_string(),
        "lb x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x00218103).to_string(), 
        "lb x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00420203).to_string(),
        "lb x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x00828283).to_string(), 
        "lb x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff30303).to_string(), 
        "lb x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x80038383).to_string(),
        "lb x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe40403).to_string(),
        "lb x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff48483).to_string(), 
        "lb x9, 2047(x9)");
}

#[test]
fn test_mcode_lh() {
    assert_eq!(
        InstructionRV::from_mcode(0x00009003).to_string(), 
        "lh x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00111083).to_string(), 
        "lh x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x00219103).to_string(), 
        "lh x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00421203).to_string(), 
        "lh x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x00829283).to_string(), 
        "lh x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff31303).to_string(), 
        "lh x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x80039383).to_string(), 
        "lh x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe41403).to_string(), 
        "lh x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff49483).to_string(), 
        "lh x9, 2047(x9)"); 
}

#[test]
fn test_mcode_lw() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000a003).to_string(), 
        "lw x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00112083).to_string(), 
        "lw x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x0021a103).to_string(), 
        "lw x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00422203).to_string(), 
        "lw x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x0082a283).to_string(), 
        "lw x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff32303).to_string(), 
        "lw x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x8003a383).to_string(), 
        "lw x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe42403).to_string(), 
        "lw x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff4a483).to_string(), 
        "lw x9, 2047(x9)");
}

#[test]
fn test_mcode_ld() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000b003).to_string(), 
        "ld x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00113083).to_string(), 
        "ld x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x0021b103).to_string(), 
        "ld x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00423203).to_string(), 
        "ld x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x0082b283).to_string(), 
        "ld x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff33303).to_string(), 
        "ld x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x8003b383).to_string(), 
        "ld x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe43403).to_string(), 
        "ld x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff4b483).to_string(), 
        "ld x9, 2047(x9)");
}

#[test]
fn test_mcode_lbu() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000c003).to_string(), 
    "lbu x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00114083).to_string(), 
    "lbu x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x0021c103).to_string(), 
    "lbu x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00424203).to_string(), 
    "lbu x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x0082c283).to_string(), 
    "lbu x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff34303).to_string(), 
    "lbu x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x8003c383).to_string(), 
    "lbu x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe44403).to_string(), 
    "lbu x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff4c483).to_string(), 
    "lbu x9, 2047(x9)");
}

#[test]
fn test_mcode_lhu() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000d003).to_string(), 
        "lhu x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00115083).to_string(), 
        "lhu x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x0021d103).to_string(), 
        "lhu x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00425203).to_string(), 
        "lhu x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x0082d283).to_string(), 
        "lhu x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff35303).to_string(), 
        "lhu x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x8003d383).to_string(), 
        "lhu x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe45403).to_string(), 
        "lhu x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff4d483).to_string(), 
        "lhu x9, 2047(x9)");
}

#[test]
fn test_mcode_lwu() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000e003).to_string(), 
        "lwu x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0x00116083).to_string(), 
        "lwu x1, 1(x2)");
    assert_eq!(
        InstructionRV::from_mcode(0x0021e103).to_string(), 
        "lwu x2, 2(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x00426203).to_string(), 
        "lwu x4, 4(x4)");
    assert_eq!(
        InstructionRV::from_mcode(0x0082e283).to_string(), 
        "lwu x5, 8(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff36303).to_string(), 
        "lwu x6, -1(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x8003e383).to_string(), 
        "lwu x7, -2048(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe46403).to_string(), 
        "lwu x8, -2(x8)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff4e483).to_string(), 
        "lwu x9, 2047(x9)");
}

#[test]
fn test_mcode_add() {
    assert_eq!(
        InstructionRV::from_mcode(0x00208033).to_string(), 
        "add x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005201b3).to_string(), 
        "add x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x00838333).to_string(), 
        "add x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b504b3).to_string(), 
        "add x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e60633).to_string(), 
        "add x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011807b3).to_string(), 
        "add x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x01498933).to_string(), 
        "add x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b0ab3).to_string(), 
        "add x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01ac8c33).to_string(), 
        "add x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de0db3).to_string(), 
        "add x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ff8f33).to_string(), 
        "add x30, x31, x31");
}

#[test]
fn test_mcode_sub() {
    //assert_eq!(disassemble(0x40208033), "sub x0, x1, x2");
    //assert_eq!(disassemble(0x405201b3), "sub x3, x4, x5");
    //assert_eq!(disassemble(0x40838333), "sub x6, x7, x8");
    //assert_eq!(disassemble(0x40b504b3), "sub x9, x10, x11");
    //assert_eq!(disassemble(0x40e60633), "sub x12, x12, x14");
    //assert_eq!(disassemble(0x411807b3), "sub x15, x16, x17");
    //assert_eq!(disassemble(0x41498933), "sub x18, x19, x20");
    //assert_eq!(disassemble(0x417b0ab3), "sub x21, x22, x23");
    //assert_eq!(disassemble(0x41ac8c33), "sub x24, x25, x26");
    //assert_eq!(disassemble(0x41de0db3), "sub x27, x28, x29");
    //assert_eq!(disassemble(0x41ff8f33), "sub x30, x31, x31");
}
