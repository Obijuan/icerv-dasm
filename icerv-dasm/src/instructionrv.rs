use crate::{mcode, regs::Reg, opcoderv::OpcodeRV};

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



    Unknown, //-- Instrucción desconocida
}


impl InstructionRV {
    pub fn from_mcode(mcode: u32) -> Self {

        let mcode = mcode::MCode::new(mcode);

        match mcode.opcode() {
            OpcodeRV::TipoIArith => {

                let func3 = mcode.func3();
                let imm = mcode.imm12();
                //-- Caso especial: srli y srai tienen el mismo codigo func3
                //-- Se diferenencias por el bit 30 del opcode (bit 10 del valor imm)
                let bit_srali:u32 = imm as u32 >> 10; 

                //-- Gestionar caso especial
                if (bit_srali==1) && (func3==0b101) {
                    return Self::Srai {
                        rd: mcode.rd(),
                        rs1: mcode.rs1(),
                        imm: imm
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
            Self::Unknown => {
                "Unknown Instruction".to_string()
            },
            _ => {
                "Not implemented".to_string()
            }
        }
    }
}

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

