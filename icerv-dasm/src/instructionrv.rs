use crate::{mcode::MCode, regs::Reg, opcoderv::OpcodeRV};


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
    Add {rd: Reg, rs1: Reg, rs2: Reg},  //-- add rd, rs1, rs2
    Sub {rd: Reg, rs1: Reg, rs2: Reg},  //-- sub rd, rs1, rs2
    Sll {rd: Reg, rs1: Reg, rs2: Reg},  //-- sll rd, rs1, rs2
    Slt {rd: Reg, rs1: Reg, rs2: Reg},  //-- slt rd, rs1, rs2
    Sltu {rd: Reg, rs1: Reg, rs2: Reg}, //-- sltu rd, rs1, rs2
    Xor {rd: Reg, rs1: Reg, rs2: Reg},  //-- xor rd, rs1, rs2
    Srl {rd: Reg, rs1: Reg, rs2: Reg},  //-- srl rd, rs1, rs2
    Or {rd: Reg, rs1: Reg, rs2: Reg},   //-- or rd, rs1, rs2
    And {rd: Reg, rs1: Reg, rs2: Reg},   //-- and rd, rs1, rs2
    Sra {rd: Reg, rs1: Reg, rs2: Reg},  //-- sra rd, rs1, rs2

    //──────────────────────────────────
    //  Instrucciones tipo S
    //──────────────────────────────────
    Sb {rs2: Reg, offs: i32, rs1: Reg}, //-- sb rs2, offs(rs1)
    Sh {rs2: Reg, offs: i32, rs1: Reg}, //-- sh rs2, offs(rs1)
    Sw {rs2: Reg, offs: i32, rs1: Reg}, //-- sw rs2, offs(rs1)
    Sd {rs2: Reg, offs: i32, rs1: Reg}, //-- sd rs2, offs(rs1)

    //──────────────────────────────────
    //  Instrucciones tipo B
    //──────────────────────────────────
    Beq {rs1: Reg, rs2: Reg, offs: i32}, //-- beq rs1, rs2, offs
    Bne {rs1: Reg, rs2: Reg, offs: i32}, //-- bne rs1, rs2, offs
    Blt {rs1: Reg, rs2: Reg, offs: i32}, //-- blt rs1, rs2, offs
    Bge {rs1: Reg, rs2: Reg, offs: i32}, //-- bge rs1, rs2, offs
    Bltu {rs1: Reg, rs2: Reg, offs: i32}, //-- bltu rs1, rs2, offs
    Bgeu {rs1: Reg, rs2: Reg, offs: i32}, //-- bgeu rs1, rs2, offs

    //──────────────────────────────────
    //  Instrucciones tipo U: LUI
    //──────────────────────────────────
    Lui {rd: Reg, imm: i32},   //-- lui rd, imm
    Auipc {rd: Reg, imm: i32}, //-- auipc rd, imm

    //──────────────────────────────────
    //  Instrucciones tipo J
    //──────────────────────────────────
    Jal {rd: Reg, offs: i32},  //-- jal rd, offs
    Jalr {rd: Reg, offs: i32, rs1: Reg},  //-- jalr rd, offs(rs1)

    //──────────────────────────────────
    //  Instrucciones tipo ecall
    //──────────────────────────────────
    Ecall,
    Ebreak,

    Unknown, //-- Instrucción desconocida
}

impl InstructionRV {
    pub fn from_mcode(mcode: u32) -> Self {

        let mcode = MCode::new(mcode);
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
                    match func3 {
                        0b_000 => 
                            Self::Sub {
                                rd: mcode.rd(),
                                rs1: mcode.rs1(),
                                rs2: mcode.rs2()
                            },
                        0b_101 => 
                            Self::Sra {
                                rd: mcode.rd(),
                                rs1: mcode.rs1(),
                                rs2: mcode.rs2()
                            },
                        _ => Self::Unknown
                    }
                } else {
                    //-- Resto de instrucciones tipo R
                    match func3 {
                        0b_000 => 
                            Self::Add { 
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_001 => 
                            Self::Sll { 
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_010 => 
                            Self::Slt { 
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_011 => 
                            Self::Sltu { 
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_100 => 
                            Self::Xor { 
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_101 => 
                            Self::Srl {
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_110 => 
                            Self::Or {
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        0b_111 => 
                            Self::And {
                                rd: mcode.rd(), 
                                rs1: mcode.rs1(), 
                                rs2: mcode.rs2() 
                            },
                        _ =>  Self::Unknown  
                    }
                }

            },
            OpcodeRV::TipoS => {
                let func3 = mcode.func3();
                match func3 {
                    0b_000 => 
                        Self::Sb {
                            rs2: mcode.rs2(),
                            offs: mcode.offset_s(),
                            rs1: mcode.rs1(),
                        },
                    0b_001 =>
                        Self::Sh {
                            rs2: mcode.rs2(),
                            offs: mcode.offset_s(),
                            rs1: mcode.rs1(),
                        },
                    0b_010 =>
                        Self::Sw {
                            rs2: mcode.rs2(),
                            offs: mcode.offset_s(),
                            rs1: mcode.rs1(),
                        },
                    0b_011 =>
                        Self::Sd {
                            rs2: mcode.rs2(),
                            offs: mcode.offset_s(),
                            rs1: mcode.rs1(),
                        },
                    _ => Self::Unknown
                }
            },
            OpcodeRV::TipoB => {
                let func3 = mcode.func3();
                match func3 {
                    0b_000 => 
                        Self::Beq {
                            rs1: mcode.rs1(),
                            rs2: mcode.rs2(),
                            offs: mcode.offset_b(),
                        },
                    0b_001 =>
                        Self::Bne {
                            rs1: mcode.rs1(),
                            rs2: mcode.rs2(),
                            offs: mcode.offset_b(),
                        },
                    0b_100 =>
                        Self::Blt {
                            rs1: mcode.rs1(),
                            rs2: mcode.rs2(),
                            offs: mcode.offset_b(),
                        },
                    0b_101 =>
                        Self::Bge {
                            rs1: mcode.rs1(),
                            rs2: mcode.rs2(),
                            offs: mcode.offset_b(),
                        },
                    0b_110 =>
                        Self::Bltu {
                            rs1: mcode.rs1(),
                            rs2: mcode.rs2(),
                            offs: mcode.offset_b(),
                        },
                    0b_111 =>
                        Self::Bgeu {
                            rs1: mcode.rs1(),
                            rs2: mcode.rs2(),
                            offs: mcode.offset_b(),
                        },
                    _ => Self::Unknown
                }
            },
            OpcodeRV::TipoULui => {
                Self::Lui {rd: mcode.rd(), imm: mcode.imm20()}
            },
            OpcodeRV::TipoUAuipc => {
                Self::Auipc {rd: mcode.rd(), imm: mcode.imm20()}
            },
            OpcodeRV::TipoJJal => {
                Self::Jal { rd: mcode.rd(), offs: mcode.offset_jal() }
            },
            OpcodeRV::TipoJJalr => {
                Self::Jalr {
                    rd: mcode.rd(), 
                    offs: mcode.imm12(), 
                    rs1: mcode.rs1()
                }
            },
            OpcodeRV::TipoEcallEbreak => {
                let imm: i32 = mcode.imm12();
                //-- Instrucción ecall o ebreak
                if imm == 0 {
                    Self::Ecall
                } else if imm == 1 {
                    Self::Ebreak
                } else {
                    Self::Unknown
                }
            }
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
            Self::Lb { rd, offs, rs1} => {
                format!("lb {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lh { rd, offs, rs1} => {
                format!("lh {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lw { rd, offs, rs1} => {
                format!("lw {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Ld { rd, offs, rs1} => {
                format!("ld {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lbu { rd, offs, rs1} => {
                format!("lbu {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lhu { rd, offs, rs1} => {
                format!("lhu {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Lwu { rd, offs, rs1} => {
                format!("lwu {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Add { rd, rs1, rs2} => {
                format!("add {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Sub { rd, rs1, rs2} => {
                format!("sub {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Sll { rd, rs1, rs2} => {
                format!("sll {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Slt { rd, rs1, rs2} => {
                format!("slt {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Sltu { rd, rs1, rs2} => {
                format!("sltu {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Srl { rd, rs1, rs2} => {
                format!("srl {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Or { rd, rs1, rs2} => {
                format!("or {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Xor { rd, rs1, rs2} => {
                format!("xor {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::And { rd, rs1, rs2} => {
                format!("and {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Sra { rd, rs1, rs2} => {
                format!("sra {}, {}, {}", rd.to_str(), 
                        rs1.to_str(), rs2.to_str())
            },
            Self::Sb { rs2, offs, rs1, } => {
                format!("sb {}, {}({})", rs2.to_str(), offs, rs1.to_str())
            },
            Self::Sh { rs2, offs, rs1, } => {
                format!("sh {}, {}({})", rs2.to_str(), offs, rs1.to_str())
            },
            Self::Sw { rs2, offs, rs1, } => {
                format!("sw {}, {}({})", rs2.to_str(), offs, rs1.to_str())
            },
            Self::Sd { rs2, offs, rs1, } => {
                format!("sd {}, {}({})", rs2.to_str(), offs, rs1.to_str())
            },
            Self::Beq { rs1, rs2, offs } => {
                format!("beq {}, {}, {}", rs1.to_str(), rs2.to_str(), offs)
            },
            Self::Bne { rs1, rs2, offs } => {
                format!("bne {}, {}, {}", rs1.to_str(), rs2.to_str(), offs)
            },
            Self::Blt { rs1, rs2, offs } => {
                format!("blt {}, {}, {}", rs1.to_str(), rs2.to_str(), offs)
            },
            Self::Bge { rs1, rs2, offs } => {
                format!("bge {}, {}, {}", rs1.to_str(), rs2.to_str(), offs)
            },
            Self::Bltu { rs1, rs2, offs } => {
                format!("bltu {}, {}, {}", rs1.to_str(), rs2.to_str(), offs)
            },
            Self::Bgeu { rs1, rs2, offs } => {
                format!("bgeu {}, {}, {}", rs1.to_str(), rs2.to_str(), offs)
            },
            Self::Lui {rd, imm} => {
                format!("lui {}, {:#07X}", rd.to_str(), imm&0xFFFFF)
            },
            Self::Auipc {rd, imm} => {
                format!("auipc {}, {:#07X}", rd.to_str(), imm & 0xFFFFF)
            },
            Self::Jal {rd, offs} => {
                format!("jal {}, {}", rd.to_str(), offs)
            },
            Self::Jalr {rd, offs, rs1} => {
                format!("jalr {}, {}({})", rd.to_str(), offs, rs1.to_str())
            },
            Self::Ecall => {
                format!("ecall")
            },
            Self::Ebreak => {
                format!("ebreak")
            },

            Self::Unknown => {
                "Unknown Instruction".to_string()
            },
        }
    }

    //──────────────────────────────────────────────
    //  Convertir la instrucción a codigo máquina
    //  (Ensamblar!)
    //──────────────────────────────────────────────
    pub fn to_mcode(&self) -> u32 {
        match self {
            Self::Addi {rd, rs1, imm} => {

                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_000, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Slli {rd, rs1, imm} => {

                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_001, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Slti {rd, rs1, imm} => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_010, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Sltiu {rd, rs1, imm} => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_011, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Xori {rd, rs1, imm} => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_100, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Srli {rd, rs1, imm} => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_101, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Ori { rd, rs1, imm } => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_110, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Andi {rd, rs1, imm} => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_111, *rd as u32, *rs1 as u32, *imm as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Srai {rd, rs1, imm} => {
                //-- Construir el codigo maquina
                let mcode = MCode::new_typei_arith(
                    0b_101, *rd as u32, *rs1 as u32, 
                    *imm as u32 | 1 << 10);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Lb { rd, offs, rs1, } => {

                //-- Construir el código máquina
                let mcode = MCode::new_typei_load(
                    0b_000, *rd as u32, *rs1 as u32,
                    *offs as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Lh { rd, offs, rs1} => {
                //-- Construir el código máquina
                let mcode = MCode::new_typei_load(
                    0b_001, *rd as u32, *rs1 as u32,
                    *offs as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Lw { rd, offs, rs1} => {
                //-- Construir el código máquina
                let mcode = MCode::new_typei_load(
                    0b_010, *rd as u32, *rs1 as u32,
                    *offs as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            Self::Ld { rd, offs, rs1} => {
                //-- Construir el código máquina
                let mcode = MCode::new_typei_load(
                    0b_011, *rd as u32, *rs1 as u32,
                    *offs as u32);

                //-- Devolver el codigo maquina como numero
                mcode.value
            }
            _ => 0
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
fn test_instructions_ori() {

    assert_eq!(
        InstructionRV::Ori { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_string(),
        "ori x0, x0, 0");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_string(),
        "ori x1, x2, 1");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X31,rs1: Reg::X1,imm: 2 }.to_string(),
        "ori x31, x1, 2");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X30,rs1: Reg::X2,imm: 4 }.to_string(),
        "ori x30, x2, 4");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X29,rs1: Reg::X3,imm: 8 }.to_string(),
        "ori x29, x3, 8");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X28,rs1: Reg::X4,imm: 16 }.to_string(),
        "ori x28, x4, 16");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X27,rs1: Reg::X5,imm: 17 }.to_string(),
        "ori x27, x5, 17");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X26,rs1: Reg::X6,imm: 30 }.to_string(),
        "ori x26, x6, 30");
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X25,rs1: Reg::X7,imm: 31 }.to_string(),
        "ori x25, x7, 31");

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
        InstructionRV::Andi { rd: Reg::X31,rs1: Reg::X1,imm: 2 }.to_string(),
        "andi x31, x1, 2");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X30,rs1: Reg::X2,imm: 4 }.to_string(),
        "andi x30, x2, 4");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X29,rs1: Reg::X3,imm: 8 }.to_string(),
        "andi x29, x3, 8");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X28,rs1: Reg::X4,imm: 16 }.to_string(),
        "andi x28, x4, 16");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X27,rs1: Reg::X5,imm: 17 }.to_string(),
        "andi x27, x5, 17");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X26,rs1: Reg::X6,imm: 30 }.to_string(),
        "andi x26, x6, 30");
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X25,rs1: Reg::X7,imm: 31 }.to_string(),
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
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "sub x0, x1, x2");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "sub x3, x4, x5");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "sub x6, x7, x8");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "sub x9, x10, x11");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "sub x12, x12, x14");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "sub x15, x16, x17");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "sub x18, x19, x20");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "sub x21, x22, x23");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "sub x24, x25, x26");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "sub x27, x28, x29");
    assert_eq!(
        InstructionRV::Sub{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "sub x30, x31, x31");
}

#[test]
fn test_instruction_sll() {
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "sll x0, x1, x2");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "sll x3, x4, x5");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "sll x6, x7, x8");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "sll x9, x10, x11");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "sll x12, x12, x14");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "sll x15, x16, x17");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "sll x18, x19, x20");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "sll x21, x22, x23");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "sll x24, x25, x26");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "sll x27, x28, x29");
    assert_eq!(
        InstructionRV::Sll{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "sll x30, x31, x31");
}

#[test]
fn test_instruction_slt() {
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "slt x0, x1, x2");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "slt x3, x4, x5");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "slt x6, x7, x8");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "slt x9, x10, x11");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "slt x12, x12, x14");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "slt x15, x16, x17");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "slt x18, x19, x20");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "slt x21, x22, x23");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "slt x24, x25, x26");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "slt x27, x28, x29");
    assert_eq!(
        InstructionRV::Slt{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "slt x30, x31, x31");
}

#[test]
fn test_instruction_sltu() {
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "sltu x0, x1, x2");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "sltu x3, x4, x5");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "sltu x6, x7, x8");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "sltu x9, x10, x11");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "sltu x12, x12, x14");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "sltu x15, x16, x17");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "sltu x18, x19, x20");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "sltu x21, x22, x23");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "sltu x24, x25, x26");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "sltu x27, x28, x29");
    assert_eq!(
        InstructionRV::Sltu{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "sltu x30, x31, x31");
}


#[test]
fn test_instruction_xor() {
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "xor x0, x1, x2");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "xor x3, x4, x5");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "xor x6, x7, x8");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "xor x9, x10, x11");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "xor x12, x12, x14");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "xor x15, x16, x17");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "xor x18, x19, x20");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "xor x21, x22, x23");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "xor x24, x25, x26");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "xor x27, x28, x29");
    assert_eq!(
        InstructionRV::Xor{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "xor x30, x31, x31");  
}

#[test]
fn test_instruction_srl() {
    
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "srl x0, x1, x2");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "srl x3, x4, x5");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "srl x6, x7, x8");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "srl x9, x10, x11");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "srl x12, x12, x14");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "srl x15, x16, x17");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "srl x18, x19, x20");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "srl x21, x22, x23");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "srl x24, x25, x26");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "srl x27, x28, x29");
    assert_eq!(
        InstructionRV::Srl{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "srl x30, x31, x31");
}

#[test]
fn test_instruction_or() {
    assert_eq!(
        InstructionRV::Or{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "or x0, x1, x2");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "or x3, x4, x5");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "or x6, x7, x8");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "or x9, x10, x11");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "or x12, x12, x14");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "or x15, x16, x17");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "or x18, x19, x20");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "or x21, x22, x23");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "or x24, x25, x26");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "or x27, x28, x29");
    assert_eq!(
        InstructionRV::Or{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "or x30, x31, x31");
}


#[test]
fn test_instruction_sra() {
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(), 
        "sra x0, x1, x2");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X3, rs1: Reg::X4, rs2: Reg::X5}.to_string(), 
        "sra x3, x4, x5");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X6, rs1: Reg::X7, rs2: Reg::X8}.to_string(), 
        "sra x6, x7, x8");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X9, rs1: Reg::X10, rs2: Reg::X11}.to_string(), 
        "sra x9, x10, x11");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X12, rs1: Reg::X12, rs2: Reg::X14}.to_string(), 
        "sra x12, x12, x14");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X15, rs1: Reg::X16, rs2: Reg::X17}.to_string(), 
        "sra x15, x16, x17");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X18, rs1: Reg::X19, rs2: Reg::X20}.to_string(), 
        "sra x18, x19, x20");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X21, rs1: Reg::X22, rs2: Reg::X23}.to_string(), 
        "sra x21, x22, x23");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X24, rs1: Reg::X25, rs2: Reg::X26}.to_string(), 
        "sra x24, x25, x26");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X27, rs1: Reg::X28, rs2: Reg::X29}.to_string(), 
        "sra x27, x28, x29");
    assert_eq!(
        InstructionRV::Sra{rd: Reg::X30, rs1: Reg::X31, rs2: Reg::X31}.to_string(), 
        "sra x30, x31, x31");
}

#[test]
fn test_instruction_sb() {
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "sb x0, 0(x1)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X2, offs: -1, rs1: Reg::X3}.to_string(), 
        "sb x2, -1(x3)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X4, offs: 2047, rs1: Reg::X5}.to_string(), 
        "sb x4, 2047(x5)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X5, offs: -2048, rs1: Reg::X6}.to_string(), 
        "sb x5, -2048(x6)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X6, offs: 2, rs1: Reg::X7}.to_string(), 
        "sb x6, 2(x7)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X8, offs: 4, rs1: Reg::X9}.to_string(), 
        "sb x8, 4(x9)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X10, offs: 8, rs1: Reg::X11}.to_string(), 
        "sb x10, 8(x11)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X12, offs: 16, rs1: Reg::X13}.to_string(), 
        "sb x12, 16(x13)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X14, offs: 32, rs1: Reg::X15}.to_string(), 
        "sb x14, 32(x15)");
      assert_eq!(
        InstructionRV::Sb{rs2: Reg::X16, offs: 64, rs1: Reg::X17}.to_string(), 
        "sb x16, 64(x17)");
}

#[test]
fn test_instruction_sh() {
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "sh x0, 0(x1)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X2, offs: -1, rs1: Reg::X3}.to_string(), 
        "sh x2, -1(x3)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X4, offs: 2047, rs1: Reg::X5}.to_string(), 
        "sh x4, 2047(x5)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X5, offs: -2048, rs1: Reg::X6}.to_string(), 
        "sh x5, -2048(x6)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X6, offs: 2, rs1: Reg::X7}.to_string(), 
        "sh x6, 2(x7)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X8, offs: 4, rs1: Reg::X9}.to_string(), 
        "sh x8, 4(x9)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X10, offs: 8, rs1: Reg::X11}.to_string(), 
        "sh x10, 8(x11)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X12, offs: 16, rs1: Reg::X13}.to_string(), 
        "sh x12, 16(x13)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X14, offs: 32, rs1: Reg::X15}.to_string(), 
        "sh x14, 32(x15)");
    assert_eq!(
        InstructionRV::Sh{rs2: Reg::X16, offs: 64, rs1: Reg::X17}.to_string(), 
        "sh x16, 64(x17)");
}

#[test]
fn test_instruction_sw() {
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "sw x0, 0(x1)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X2, offs: -1, rs1: Reg::X3}.to_string(), 
        "sw x2, -1(x3)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X4, offs: 2047, rs1: Reg::X5}.to_string(), 
        "sw x4, 2047(x5)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X5, offs: -2048, rs1: Reg::X6}.to_string(), 
        "sw x5, -2048(x6)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X6, offs: 2, rs1: Reg::X7}.to_string(), 
        "sw x6, 2(x7)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X8, offs: 4, rs1: Reg::X9}.to_string(), 
        "sw x8, 4(x9)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X10, offs: 8, rs1: Reg::X11}.to_string(), 
        "sw x10, 8(x11)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X12, offs: 16, rs1: Reg::X13}.to_string(), 
        "sw x12, 16(x13)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X14, offs: 32, rs1: Reg::X15}.to_string(), 
        "sw x14, 32(x15)");
    assert_eq!(
        InstructionRV::Sw{rs2: Reg::X16, offs: 64, rs1: Reg::X17}.to_string(), 
        "sw x16, 64(x17)");
}

#[test]
fn test_instruction_sd() {
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(), 
        "sd x0, 0(x1)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X2, offs: -1, rs1: Reg::X3}.to_string(), 
        "sd x2, -1(x3)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X4, offs: 2047, rs1: Reg::X5}.to_string(), 
        "sd x4, 2047(x5)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X5, offs: -2048, rs1: Reg::X6}.to_string(), 
        "sd x5, -2048(x6)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X6, offs: 2, rs1: Reg::X7}.to_string(), 
        "sd x6, 2(x7)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X8, offs: 4, rs1: Reg::X9}.to_string(), 
        "sd x8, 4(x9)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X10, offs: 8, rs1: Reg::X11}.to_string(), 
        "sd x10, 8(x11)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X12, offs: 16, rs1: Reg::X13}.to_string(), 
        "sd x12, 16(x13)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X14, offs: 32, rs1: Reg::X15}.to_string(), 
        "sd x14, 32(x15)");
    assert_eq!(
        InstructionRV::Sd{rs2: Reg::X16, offs: 64, rs1: Reg::X17}.to_string(), 
        "sd x16, 64(x17)");
}


#[test]
fn test_instruction_beq() {
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X0, rs2: Reg::X0, offs: 0}.to_string(), 
        "beq x0, x0, 0"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(), 
        "beq x1, x2, -4"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(), 
        "beq x3, x4, -8"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(), 
        "beq x5, x6, -12"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(), 
        "beq x7, x8, 24"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(), 
        "beq x9, x10, 20"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(), 
        "beq x11, x12, 16"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X13, rs2: Reg::X14, offs: 12}.to_string(), 
        "beq x13, x14, 12"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X15, rs2: Reg::X16, offs: 8}.to_string(), 
        "beq x15, x16, 8"); 
    assert_eq!(
        InstructionRV::Beq {rs1: Reg::X17, rs2: Reg::X18, offs: 4}.to_string(), 
        "beq x17, x18, 4"); 
}

#[test]
fn test_instruction_bne() {
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X0, rs2: Reg::X0, offs: 0}.to_string(), 
        "bne x0, x0, 0");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(), 
        "bne x1, x2, -4");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(), 
        "bne x3, x4, -8");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(), 
        "bne x5, x6, -12");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(), 
        "bne x7, x8, 24");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(), 
        "bne x9, x10, 20");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(), 
        "bne x11, x12, 16");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X13, rs2: Reg::X14, offs: 12}.to_string(), 
        "bne x13, x14, 12");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X15, rs2: Reg::X16, offs: 8}.to_string(), 
        "bne x15, x16, 8");
    assert_eq!(
        InstructionRV::Bne{rs1: Reg::X17, rs2: Reg::X18, offs: 4}.to_string(), 
        "bne x17, x18, 4");
}

#[test]
fn test_instruction_blt() {
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X0, rs2: Reg::X0, offs: 0}.to_string(), 
        "blt x0, x0, 0");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(), 
        "blt x1, x2, -4");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(), 
        "blt x3, x4, -8");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(), 
        "blt x5, x6, -12");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(), 
        "blt x7, x8, 24");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(), 
        "blt x9, x10, 20");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(), 
        "blt x11, x12, 16");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X13, rs2: Reg::X14, offs: 12}.to_string(), 
        "blt x13, x14, 12");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X15, rs2: Reg::X16, offs: 8}.to_string(), 
        "blt x15, x16, 8");
    assert_eq!(
        InstructionRV::Blt{rs1: Reg::X17, rs2: Reg::X18, offs: 4}.to_string(), 
        "blt x17, x18, 4"); 
}

#[test]
fn test_instruction_bge() {
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X0, rs2: Reg::X0, offs: 0}.to_string(), 
        "bge x0, x0, 0");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(), 
        "bge x1, x2, -4");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(), 
        "bge x3, x4, -8");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(), 
        "bge x5, x6, -12");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(), 
        "bge x7, x8, 24");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(), 
        "bge x9, x10, 20");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(), 
        "bge x11, x12, 16");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X13, rs2: Reg::X14, offs: 12}.to_string(), 
        "bge x13, x14, 12");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X15, rs2: Reg::X16, offs: 8}.to_string(), 
        "bge x15, x16, 8");
    assert_eq!(
        InstructionRV::Bge{rs1: Reg::X17, rs2: Reg::X18, offs: 4}.to_string(), 
        "bge x17, x18, 4"); 
}

#[test]
fn test_instruction_bltu() {
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X0, rs2: Reg::X0, offs: 0}.to_string(), 
        "bltu x0, x0, 0");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(), 
        "bltu x1, x2, -4");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(), 
        "bltu x3, x4, -8");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(), 
        "bltu x5, x6, -12");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(), 
        "bltu x7, x8, 24");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(), 
        "bltu x9, x10, 20");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(), 
        "bltu x11, x12, 16");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X13, rs2: Reg::X14, offs: 12}.to_string(), 
        "bltu x13, x14, 12");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X15, rs2: Reg::X16, offs: 8}.to_string(), 
        "bltu x15, x16, 8");
    assert_eq!(
        InstructionRV::Bltu{rs1: Reg::X17, rs2: Reg::X18, offs: 4}.to_string(), 
        "bltu x17, x18, 4"); 
}

#[test]
fn test_instruction_bgeu() {
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X0, rs2: Reg::X0, offs: 0}.to_string(), 
        "bgeu x0, x0, 0");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(), 
        "bgeu x1, x2, -4");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(), 
        "bgeu x3, x4, -8");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(), 
        "bgeu x5, x6, -12");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(), 
        "bgeu x7, x8, 24");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(), 
        "bgeu x9, x10, 20");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(), 
        "bgeu x11, x12, 16");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X13, rs2: Reg::X14, offs: 12}.to_string(), 
        "bgeu x13, x14, 12");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X15, rs2: Reg::X16, offs: 8}.to_string(), 
        "bgeu x15, x16, 8");
    assert_eq!(
        InstructionRV::Bgeu{rs1: Reg::X17, rs2: Reg::X18, offs: 4}.to_string(), 
        "bgeu x17, x18, 4"); 
}

#[test]
fn test_instruction_lui() {
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X0, imm: 0}.to_string(), 
        "lui x0, 0x00000");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X1, imm: 1}.to_string(), 
        "lui x1, 0x00001");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X2, imm: 0x20}.to_string(), 
        "lui x2, 0x00020");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X3, imm: 0x400}.to_string(), 
        "lui x3, 0x00400");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X4, imm: 0x8000}.to_string(), 
        "lui x4, 0x08000");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X5, imm: 0x7FFFF}.to_string(), 
        "lui x5, 0x7FFFF");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X6, imm: 0x80000}.to_string(), 
        "lui x6, 0x80000");
    assert_eq!(
        InstructionRV::Lui {rd: Reg::X7, imm: 0xFFFFF}.to_string(), 
        "lui x7, 0xFFFFF");
}

#[test]
fn test_instruction_auipc() {
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X0, imm: 0}.to_string(), 
        "auipc x0, 0x00000");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X1, imm: 1}.to_string(), 
        "auipc x1, 0x00001");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X2, imm: 0x20}.to_string(), 
        "auipc x2, 0x00020");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X3, imm: 0x400}.to_string(), 
        "auipc x3, 0x00400");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X4, imm: 0x8000}.to_string(), 
        "auipc x4, 0x08000");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X5, imm: 0x7FFFF}.to_string(), 
        "auipc x5, 0x7FFFF");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X6, imm: 0x80000}.to_string(), 
        "auipc x6, 0x80000");
    assert_eq!(
        InstructionRV::Auipc {rd: Reg::X7, imm: 0xFFFFF}.to_string(), 
        "auipc x7, 0xFFFFF");
}

#[test]
fn test_instruction_jal() {
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X0, offs: 0 }.to_string(), 
    "jal x0, 0");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X1, offs: -4 }.to_string(), 
    "jal x1, -4");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X2, offs: -8 }.to_string(), 
    "jal x2, -8");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X3, offs: -12 }.to_string(), 
    "jal x3, -12");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X4, offs: -16 }.to_string(), 
    "jal x4, -16");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X5, offs: 20 }.to_string(), 
    "jal x5, 20");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X6, offs: 16 }.to_string(), 
    "jal x6, 16");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X7, offs: 12 }.to_string(), 
    "jal x7, 12");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X8, offs: 8 }.to_string(), 
    "jal x8, 8");
    assert_eq!(
    InstructionRV::Jal { rd: Reg::X9, offs: 4 }.to_string(), 
    "jal x9, 4");
}

#[test]
fn test_instruction_jalr() {
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X0, offs: 0, rs1: Reg::X0}.to_string(), 
        "jalr x0, 0(x0)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X1, offs: -1, rs1: Reg::X10}.to_string(), 
        "jalr x1, -1(x10)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X2, offs: -2, rs1: Reg::X11}.to_string(), 
        "jalr x2, -2(x11)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X3, offs: -4, rs1: Reg::X12}.to_string(), 
        "jalr x3, -4(x12)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X4, offs: -8, rs1: Reg::X13}.to_string(), 
        "jalr x4, -8(x13)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X5, offs: -2048, rs1: Reg::X14}.to_string(), 
        "jalr x5, -2048(x14)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X6, offs: 2047, rs1: Reg::X15}.to_string(), 
        "jalr x6, 2047(x15)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X7, offs: 16, rs1: Reg::X16}.to_string(), 
        "jalr x7, 16(x16)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X8, offs: 8, rs1: Reg::X17}.to_string(), 
        "jalr x8, 8(x17)");
    assert_eq!(
        InstructionRV::Jalr{rd: Reg::X9, offs: 4, rs1: Reg::X18}.to_string(), 
        "jalr x9, 4(x18)");
}

#[test]
fn test_instuction_ecall_ebreak() {
    assert_eq!(InstructionRV::Ecall.to_string(), "ecall");
    assert_eq!(InstructionRV::Ebreak.to_string(), "ebreak");
}


//─────────────────────────────────────────
//  PRUEBAS DEL CODIGO MAQUINA
//─────────────────────────────────────────
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
fn test_mcode_ori() {
    assert_eq!(
        InstructionRV::from_mcode(0x00006013).to_string(),
        "ori x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0x00116093).to_string(),
        "ori x1, x2, 1");
    assert_eq!(
        InstructionRV::from_mcode(0x0020ef93).to_string(),
        "ori x31, x1, 2");
    assert_eq!(
        InstructionRV::from_mcode(0x00416f13).to_string(),
        "ori x30, x2, 4");
    assert_eq!(
        InstructionRV::from_mcode(0x0081ee93).to_string(),
        "ori x29, x3, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01026e13).to_string(),
        "ori x28, x4, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x0112ed93).to_string(),
        "ori x27, x5, 17");
    assert_eq!(
        InstructionRV::from_mcode(0x01e36d13).to_string(),
        "ori x26, x6, 30");
    assert_eq!(
        InstructionRV::from_mcode(0x01f3ec93).to_string(),
        "ori x25, x7, 31");

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
    assert_eq!(
        InstructionRV::from_mcode(0x40208033).to_string(), 
        "sub x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x405201b3).to_string(), 
        "sub x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x40838333).to_string(), 
        "sub x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x40b504b3).to_string(), 
        "sub x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x40e60633).to_string(), 
        "sub x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x411807b3).to_string(), 
        "sub x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x41498933).to_string(), 
        "sub x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x417b0ab3).to_string(), 
        "sub x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x41ac8c33).to_string(), 
        "sub x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x41de0db3).to_string(), 
        "sub x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x41ff8f33).to_string(), 
        "sub x30, x31, x31");
}

#[test]
fn test_mcode_sll() {
    assert_eq!(
        InstructionRV::from_mcode(0x00209033).to_string(), 
        "sll x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005211b3).to_string(), 
        "sll x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x00839333).to_string(), 
        "sll x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b514b3).to_string(), 
        "sll x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e61633).to_string(), 
        "sll x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011817b3).to_string(), 
        "sll x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x01499933).to_string(), 
        "sll x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b1ab3).to_string(), 
        "sll x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01ac9c33).to_string(), 
        "sll x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de1db3).to_string(), 
        "sll x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ff9f33).to_string(), 
        "sll x30, x31, x31");
}

#[test]
fn test_mcode_slt() {
    assert_eq!(
        InstructionRV::from_mcode(0x0020a033).to_string(), 
        "slt x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005221b3).to_string(), 
        "slt x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x0083a333).to_string(), 
        "slt x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b524b3).to_string(), 
        "slt x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e62633).to_string(), 
        "slt x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011827b3).to_string(), 
        "slt x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x0149a933).to_string(), 
        "slt x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b2ab3).to_string(), 
        "slt x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01acac33).to_string(), 
        "slt x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de2db3).to_string(), 
        "slt x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ffaf33).to_string(), 
        "slt x30, x31, x31");
}

#[test]
fn test_mcode_sltu() {
    assert_eq!(
        InstructionRV::from_mcode(0x0020b033).to_string(), 
        "sltu x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005231b3).to_string(), 
        "sltu x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x0083b333).to_string(), 
        "sltu x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b534b3).to_string(), 
        "sltu x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e63633).to_string(), 
        "sltu x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011837b3).to_string(), 
        "sltu x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x0149b933).to_string(), 
        "sltu x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b3ab3).to_string(), 
        "sltu x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01acbc33).to_string(), 
        "sltu x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de3db3).to_string(), 
        "sltu x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ffbf33).to_string(), 
        "sltu x30, x31, x31");
}

#[test]
fn test_mcode_xor() {
    assert_eq!(
        InstructionRV::from_mcode(0x0020c033).to_string(), 
        "xor x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005241b3).to_string(), 
        "xor x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x0083c333).to_string(), 
        "xor x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b544b3).to_string(), 
        "xor x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e64633).to_string(), 
        "xor x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011847b3).to_string(), 
        "xor x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x0149c933).to_string(), 
        "xor x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b4ab3).to_string(), 
        "xor x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01accc33).to_string(), 
        "xor x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de4db3).to_string(), 
        "xor x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ffcf33).to_string(), 
        "xor x30, x31, x31");  
}

#[test]
fn test_mcode_srl() {
    assert_eq!(
        InstructionRV::from_mcode(0x0020d033).to_string(), 
        "srl x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005251b3).to_string(), 
        "srl x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x0083d333).to_string(), 
        "srl x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b554b3).to_string(), 
        "srl x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e65633).to_string(), 
        "srl x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011857b3).to_string(), 
        "srl x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x0149d933).to_string(), 
        "srl x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b5ab3).to_string(), 
        "srl x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01acdc33).to_string(), 
        "srl x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de5db3).to_string(), 
        "srl x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ffdf33).to_string(), 
        "srl x30, x31, x31");
}

#[test]
fn test_mcode_or() {
    assert_eq!(
        InstructionRV::from_mcode(0x0020e033).to_string(), 
        "or x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x005261b3).to_string(), 
        "or x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x0083e333).to_string(), 
        "or x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x00b564b3).to_string(), 
        "or x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x00e66633).to_string(), 
        "or x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x011867b3).to_string(), 
        "or x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x0149e933).to_string(), 
        "or x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x017b6ab3).to_string(), 
        "or x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x01acec33).to_string(), 
        "or x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x01de6db3).to_string(), 
        "or x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x01ffef33).to_string(), 
        "or x30, x31, x31");
}

#[test]
fn test_mcode_sra() {
    assert_eq!(
        InstructionRV::from_mcode(0x4020d033).to_string(), 
        "sra x0, x1, x2");
    assert_eq!(
        InstructionRV::from_mcode(0x405251b3).to_string(), 
        "sra x3, x4, x5");
    assert_eq!(
        InstructionRV::from_mcode(0x4083d333).to_string(), 
        "sra x6, x7, x8");
    assert_eq!(
        InstructionRV::from_mcode(0x40b554b3).to_string(), 
        "sra x9, x10, x11");
    assert_eq!(
        InstructionRV::from_mcode(0x40e65633).to_string(), 
        "sra x12, x12, x14");
    assert_eq!(
        InstructionRV::from_mcode(0x411857b3).to_string(), 
        "sra x15, x16, x17");
    assert_eq!(
        InstructionRV::from_mcode(0x4149d933).to_string(), 
        "sra x18, x19, x20");
    assert_eq!(
        InstructionRV::from_mcode(0x417b5ab3).to_string(), 
        "sra x21, x22, x23");
    assert_eq!(
        InstructionRV::from_mcode(0x41acdc33).to_string(), 
        "sra x24, x25, x26");
    assert_eq!(
        InstructionRV::from_mcode(0x41de5db3).to_string(), 
        "sra x27, x28, x29");
    assert_eq!(
        InstructionRV::from_mcode(0x41ffdf33).to_string(), 
        "sra x30, x31, x31");
}

#[test]
fn test_mcode_sb() {
      assert_eq!(
        InstructionRV::from_mcode(0x00008023).to_string(), 
        "sb x0, 0(x1)");
      assert_eq!(
        InstructionRV::from_mcode(0xfe218fa3).to_string(), 
        "sb x2, -1(x3)");
      assert_eq!(
        InstructionRV::from_mcode(0x7e428fa3).to_string(), 
        "sb x4, 2047(x5)");
      assert_eq!(
        InstructionRV::from_mcode(0x80530023).to_string(), 
        "sb x5, -2048(x6)");
      assert_eq!(
        InstructionRV::from_mcode(0x00638123).to_string(), 
        "sb x6, 2(x7)");
      assert_eq!(
        InstructionRV::from_mcode(0x00848223).to_string(), 
        "sb x8, 4(x9)");
      assert_eq!(
        InstructionRV::from_mcode(0x00a58423).to_string(), 
        "sb x10, 8(x11)");
      assert_eq!(
        InstructionRV::from_mcode(0x00c68823).to_string(), 
        "sb x12, 16(x13)");
      assert_eq!(
        InstructionRV::from_mcode(0x02e78023).to_string(), 
        "sb x14, 32(x15)");
      assert_eq!(
        InstructionRV::from_mcode(0x05088023).to_string(), 
        "sb x16, 64(x17)");
}


#[test]
fn test_mcode_sh() {
    assert_eq!(
        InstructionRV::from_mcode(0x00009023).to_string(), 
        "sh x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0xfe219fa3).to_string(), 
        "sh x2, -1(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x7e429fa3).to_string(), 
        "sh x4, 2047(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0x80531023).to_string(), 
        "sh x5, -2048(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x00639123).to_string(), 
        "sh x6, 2(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0x00849223).to_string(), 
        "sh x8, 4(x9)");
    assert_eq!(
        InstructionRV::from_mcode(0x00a59423).to_string(), 
        "sh x10, 8(x11)");
    assert_eq!(
        InstructionRV::from_mcode(0x00c69823).to_string(), 
        "sh x12, 16(x13)");
    assert_eq!(
        InstructionRV::from_mcode(0x02e79023).to_string(), 
        "sh x14, 32(x15)");
    assert_eq!(
        InstructionRV::from_mcode(0x05089023).to_string(), 
        "sh x16, 64(x17)");
}

#[test]
fn test_mcode_sw() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000a023).to_string(), 
        "sw x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0xfe21afa3).to_string(), 
        "sw x2, -1(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x7e42afa3).to_string(), 
        "sw x4, 2047(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0x80532023).to_string(), 
        "sw x5, -2048(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x0063a123).to_string(), 
        "sw x6, 2(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0x0084a223).to_string(), 
        "sw x8, 4(x9)");
    assert_eq!(
        InstructionRV::from_mcode(0x00a5a423).to_string(), 
        "sw x10, 8(x11)");
    assert_eq!(
        InstructionRV::from_mcode(0x00c6a823).to_string(), 
        "sw x12, 16(x13)");
    assert_eq!(
        InstructionRV::from_mcode(0x02e7a023).to_string(), 
        "sw x14, 32(x15)");
    assert_eq!(
        InstructionRV::from_mcode(0x0508a023).to_string(), 
        "sw x16, 64(x17)");
}

#[test]
fn test_mcode_sd() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000b023).to_string(), 
        "sd x0, 0(x1)");
    assert_eq!(
        InstructionRV::from_mcode(0xfe21bfa3).to_string(), 
        "sd x2, -1(x3)");
    assert_eq!(
        InstructionRV::from_mcode(0x7e42bfa3).to_string(), 
        "sd x4, 2047(x5)");
    assert_eq!(
        InstructionRV::from_mcode(0x80533023).to_string(), 
        "sd x5, -2048(x6)");
    assert_eq!(
        InstructionRV::from_mcode(0x0063b123).to_string(), 
        "sd x6, 2(x7)");
    assert_eq!(
        InstructionRV::from_mcode(0x0084b223).to_string(), 
        "sd x8, 4(x9)");
    assert_eq!(
        InstructionRV::from_mcode(0x00a5b423).to_string(), 
        "sd x10, 8(x11)");
    assert_eq!(
        InstructionRV::from_mcode(0x00c6b823).to_string(), 
        "sd x12, 16(x13)");
    assert_eq!(
        InstructionRV::from_mcode(0x02e7b023).to_string(), 
        "sd x14, 32(x15)");
    assert_eq!(
        InstructionRV::from_mcode(0x0508b023).to_string(), 
        "sd x16, 64(x17)");
}

#[test]
fn test_mcode_beq() {
    assert_eq!(
        InstructionRV::from_mcode(0x00000063).to_string(), 
        "beq x0, x0, 0"); 
    assert_eq!(
        InstructionRV::from_mcode(0xfe208ee3).to_string(), 
        "beq x1, x2, -4"); 
    assert_eq!(
        InstructionRV::from_mcode(0xfe418ce3).to_string(), 
        "beq x3, x4, -8"); 
    assert_eq!(
        InstructionRV::from_mcode(0xfe628ae3).to_string(), 
        "beq x5, x6, -12"); 
    assert_eq!(
        InstructionRV::from_mcode(0x00838c63).to_string(), 
        "beq x7, x8, 24"); 
    assert_eq!(
        InstructionRV::from_mcode(0x00a48a63).to_string(), 
        "beq x9, x10, 20"); 
    assert_eq!(
        InstructionRV::from_mcode(0x00c58863).to_string(), 
        "beq x11, x12, 16"); 
    assert_eq!(
        InstructionRV::from_mcode(0x00e68663).to_string(), 
        "beq x13, x14, 12"); 
    assert_eq!(
        InstructionRV::from_mcode(0x01078463).to_string(), 
        "beq x15, x16, 8"); 
    assert_eq!(
        InstructionRV::from_mcode(0x01288263).to_string(), 
        "beq x17, x18, 4"); 
}

#[test]
fn test_mcode_bne() {
    assert_eq!(
        InstructionRV::from_mcode(0x00001063).to_string(), 
        "bne x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0xfe209ee3).to_string(), 
        "bne x1, x2, -4");
    assert_eq!(
        InstructionRV::from_mcode(0xfe419ce3).to_string(), 
        "bne x3, x4, -8");
    assert_eq!(
        InstructionRV::from_mcode(0xfe629ae3).to_string(), 
        "bne x5, x6, -12");
    assert_eq!(
        InstructionRV::from_mcode(0x00839c63).to_string(), 
        "bne x7, x8, 24");
    assert_eq!(
        InstructionRV::from_mcode(0x00a49a63).to_string(), 
        "bne x9, x10, 20");
    assert_eq!(
        InstructionRV::from_mcode(0x00c59863).to_string(), 
        "bne x11, x12, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x00e69663).to_string(), 
        "bne x13, x14, 12");
    assert_eq!(
        InstructionRV::from_mcode(0x01079463).to_string(), 
        "bne x15, x16, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x01289263).to_string(), 
        "bne x17, x18, 4");
}

#[test]
fn test_mcode_blt() {
    assert_eq!(
        InstructionRV::from_mcode(0x00004063).to_string(), 
        "blt x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0xfe20cee3).to_string(), 
        "blt x1, x2, -4");
    assert_eq!(
        InstructionRV::from_mcode(0xfe41cce3).to_string(), 
        "blt x3, x4, -8");
    assert_eq!(
        InstructionRV::from_mcode(0xfe62cae3).to_string(), 
        "blt x5, x6, -12");
    assert_eq!(
        InstructionRV::from_mcode(0x0083cc63).to_string(), 
        "blt x7, x8, 24");
    assert_eq!(
        InstructionRV::from_mcode(0x00a4ca63).to_string(), 
        "blt x9, x10, 20");
    assert_eq!(
        InstructionRV::from_mcode(0x00c5c863).to_string(), 
        "blt x11, x12, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x00e6c663).to_string(), 
        "blt x13, x14, 12");
    assert_eq!(
        InstructionRV::from_mcode(0x0107c463).to_string(), 
        "blt x15, x16, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x0128c263).to_string(), 
        "blt x17, x18, 4"); 
}

#[test]
fn test_mcode_bge() {
    assert_eq!(
        InstructionRV::from_mcode(0x00005063).to_string(), 
        "bge x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0xfe20dee3).to_string(), 
        "bge x1, x2, -4");
    assert_eq!(
        InstructionRV::from_mcode(0xfe41dce3).to_string(), 
        "bge x3, x4, -8");
    assert_eq!(
        InstructionRV::from_mcode(0xfe62dae3).to_string(), 
        "bge x5, x6, -12");
    assert_eq!(
        InstructionRV::from_mcode(0x0083dc63).to_string(), 
        "bge x7, x8, 24");
    assert_eq!(
        InstructionRV::from_mcode(0x00a4da63).to_string(), 
        "bge x9, x10, 20");
    assert_eq!(
        InstructionRV::from_mcode(0x00c5d863).to_string(), 
        "bge x11, x12, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x00e6d663).to_string(), 
        "bge x13, x14, 12");
    assert_eq!(
        InstructionRV::from_mcode(0x0107d463).to_string(), 
        "bge x15, x16, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x0128d263).to_string(), 
        "bge x17, x18, 4"); 
}

#[test]
fn test_mcode_bltu() {
    assert_eq!(
        InstructionRV::from_mcode(0x00006063).to_string(), 
        "bltu x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0xfe20eee3).to_string(), 
        "bltu x1, x2, -4");
    assert_eq!(
        InstructionRV::from_mcode(0xfe41ece3).to_string(), 
        "bltu x3, x4, -8");
    assert_eq!(
        InstructionRV::from_mcode(0xfe62eae3).to_string(), 
        "bltu x5, x6, -12");
    assert_eq!(
        InstructionRV::from_mcode(0x0083ec63).to_string(), 
        "bltu x7, x8, 24");
    assert_eq!(
        InstructionRV::from_mcode(0x00a4ea63).to_string(), 
        "bltu x9, x10, 20");
    assert_eq!(
        InstructionRV::from_mcode(0x00c5e863).to_string(), 
        "bltu x11, x12, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x00e6e663).to_string(), 
        "bltu x13, x14, 12");
    assert_eq!(
        InstructionRV::from_mcode(0x0107e463).to_string(), 
        "bltu x15, x16, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x0128e263).to_string(), 
        "bltu x17, x18, 4"); 
}

#[test]
fn test_mcode_bgeu() {
    assert_eq!(
        InstructionRV::from_mcode(0x00007063).to_string(), 
        "bgeu x0, x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0xfe20fee3).to_string(), 
        "bgeu x1, x2, -4");
    assert_eq!(
        InstructionRV::from_mcode(0xfe41fce3).to_string(), 
        "bgeu x3, x4, -8");
    assert_eq!(
        InstructionRV::from_mcode(0xfe62fae3).to_string(), 
        "bgeu x5, x6, -12");
    assert_eq!(
        InstructionRV::from_mcode(0x0083fc63).to_string(), 
        "bgeu x7, x8, 24");
    assert_eq!(
        InstructionRV::from_mcode(0x00a4fa63).to_string(), 
        "bgeu x9, x10, 20");
    assert_eq!(
        InstructionRV::from_mcode(0x00c5f863).to_string(), 
        "bgeu x11, x12, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x00e6f663).to_string(), 
        "bgeu x13, x14, 12");
    assert_eq!(
        InstructionRV::from_mcode(0x0107f463).to_string(), 
        "bgeu x15, x16, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x0128f263).to_string(), 
        "bgeu x17, x18, 4"); 
}

#[test]
fn test_mcode_lui() {
    assert_eq!(
        InstructionRV::from_mcode(0x00000037).to_string(), 
        "lui x0, 0x00000");
    assert_eq!(
        InstructionRV::from_mcode(0x000010b7).to_string(), 
        "lui x1, 0x00001");
    assert_eq!(
        InstructionRV::from_mcode(0x00020137).to_string(), 
        "lui x2, 0x00020");
    assert_eq!(
        InstructionRV::from_mcode(0x004001b7).to_string(), 
        "lui x3, 0x00400");
    assert_eq!(
        InstructionRV::from_mcode(0x08000237).to_string(), 
        "lui x4, 0x08000");
    assert_eq!(
        InstructionRV::from_mcode(0x7ffff2b7).to_string(), 
        "lui x5, 0x7FFFF");
    assert_eq!(
        InstructionRV::from_mcode(0x80000337).to_string(), 
        "lui x6, 0x80000");
    assert_eq!(
        InstructionRV::from_mcode(0xfffff3b7).to_string(), 
        "lui x7, 0xFFFFF");
}

#[test]
fn test_mcode_auipc() {
    assert_eq!(
        InstructionRV::from_mcode(0x00000017).to_string(), 
        "auipc x0, 0x00000");
    assert_eq!(
        InstructionRV::from_mcode(0x00001097).to_string(), 
        "auipc x1, 0x00001");
    assert_eq!(
        InstructionRV::from_mcode(0x00020117).to_string(), 
        "auipc x2, 0x00020");
    assert_eq!(
        InstructionRV::from_mcode(0x00400197).to_string(), 
        "auipc x3, 0x00400");
    assert_eq!(
        InstructionRV::from_mcode(0x08000217).to_string(), 
        "auipc x4, 0x08000");
    assert_eq!(
        InstructionRV::from_mcode(0x7ffff297).to_string(), 
        "auipc x5, 0x7FFFF");
    assert_eq!(
        InstructionRV::from_mcode(0x80000317).to_string(), 
        "auipc x6, 0x80000");
    assert_eq!(
        InstructionRV::from_mcode(0xfffff397).to_string(), 
        "auipc x7, 0xFFFFF");
}

#[test]
fn test_mcode_jal() {
    assert_eq!(
        InstructionRV::from_mcode(0x0000006f).to_string(), 
        "jal x0, 0");
    assert_eq!(
        InstructionRV::from_mcode(0xffdff0ef).to_string(), 
        "jal x1, -4");
    assert_eq!(
        InstructionRV::from_mcode(0xff9ff16f).to_string(), 
        "jal x2, -8");
    assert_eq!(
        InstructionRV::from_mcode(0xff5ff1ef).to_string(), 
        "jal x3, -12");
    assert_eq!(
        InstructionRV::from_mcode(0xff1ff26f).to_string(), 
        "jal x4, -16");
    assert_eq!(
        InstructionRV::from_mcode(0x014002ef).to_string(), 
        "jal x5, 20");
    assert_eq!(
        InstructionRV::from_mcode(0x0100036f).to_string(), 
        "jal x6, 16");
    assert_eq!(
        InstructionRV::from_mcode(0x00c003ef).to_string(), 
        "jal x7, 12");
    assert_eq!(
        InstructionRV::from_mcode(0x0080046f).to_string(), 
        "jal x8, 8");
    assert_eq!(
        InstructionRV::from_mcode(0x004004ef).to_string(), 
        "jal x9, 4");
}

#[test]
fn test_mcode_jalr() {
    assert_eq!(
        InstructionRV::from_mcode(0x00000067).to_string(), 
        "jalr x0, 0(x0)");
    assert_eq!(
        InstructionRV::from_mcode(0xfff500e7).to_string(), 
        "jalr x1, -1(x10)");
    assert_eq!(
        InstructionRV::from_mcode(0xffe58167).to_string(), 
        "jalr x2, -2(x11)");
    assert_eq!(
        InstructionRV::from_mcode(0xffc601e7).to_string(), 
        "jalr x3, -4(x12)");
    assert_eq!(
        InstructionRV::from_mcode(0xff868267).to_string(), 
        "jalr x4, -8(x13)");
    assert_eq!(
        InstructionRV::from_mcode(0x800702e7).to_string(), 
        "jalr x5, -2048(x14)");
    assert_eq!(
        InstructionRV::from_mcode(0x7ff78367).to_string(), 
        "jalr x6, 2047(x15)");
    assert_eq!(
        InstructionRV::from_mcode(0x010803e7).to_string(), 
        "jalr x7, 16(x16)");
    assert_eq!(
        InstructionRV::from_mcode(0x00888467).to_string(), 
        "jalr x8, 8(x17)");
    assert_eq!(
        InstructionRV::from_mcode(0x004904e7).to_string(), 
        "jalr x9, 4(x18)");
}

#[test]
fn test_disassemble_ecall_ebreak() {
    assert_eq!(InstructionRV::from_mcode(0x00000073).to_string(), "ecall");
    assert_eq!(InstructionRV::from_mcode(0x00100073).to_string(), "ebreak");
}


//────────────────────────────────────────────────
//  Pruebas del metodo .to_mcode()
//────────────────────────────────────────────────
#[test]
fn test_mcode2_addi() {
    //────────────────────────────────────────────────
    //  Test de la instrucción ADDI
    //────────────────────────────────────────────────
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00000013);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 1 }.to_mcode(),
        0x00100093);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X2, rs1: Reg::X0, imm: 2 }.to_mcode(),
        0x00200113);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X3, rs1: Reg::X0, imm: -1 }.to_mcode(),
        0xfff00193);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X4, rs1: Reg::X0, imm: 2047 }.to_mcode(),
        0x7ff00213);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X31, rs1: Reg::X1, imm: 3 }.to_mcode(),
        0x00308f93); 
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X8, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00410413);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X16, rs1: Reg::X4, imm: 8 }.to_mcode(),
        0x00820813);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: 16 }.to_mcode(),
        0x01040893);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: -16 }.to_mcode(),
        0xff040893);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X17, rs1: Reg::X8, imm: -2048 }.to_mcode(),
        0x80040893);
    assert_eq!(
        InstructionRV::Addi { rd: Reg::X1, rs1: Reg::X0, imm: 170 }.to_mcode(),
        0x0aa00093);  
}   

#[test]
fn test_mcode2_slli() {

    assert_eq!(
        InstructionRV::Slli { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00001013);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00111093);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x00209f93);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00411f13);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x00819e93);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01021e13);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x01129d93);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e31d13);
    assert_eq!(
        InstructionRV::Slli { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f39c93);

}

#[test]
fn test_mcode2_slti() {
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00002013);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00112093);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x0020af93);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00412f13);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x0081ae93);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01022e13);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x0112ad93);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e32d13);
    assert_eq!(
        InstructionRV::Slti { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f3ac93);

}

#[test]
fn test_mcode2_sltiu() {
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00003013);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00113093);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x0020bf93);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00413f13);
    assert_eq!(             
        InstructionRV::Sltiu { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x0081be93);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01023e13);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x0112bd93);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e33d13);
    assert_eq!(
        InstructionRV::Sltiu { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f3bc93);

}

#[test]
fn test_mcode2_xori() {

    assert_eq!(
        InstructionRV::Xori { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00004013);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00114093);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x0020cf93);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00414f13);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x0081ce93);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01024e13);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x0112cd93);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e34d13);
    assert_eq!(
        InstructionRV::Xori { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f3cc93);

}

#[test]
fn test_mcode2_srli() {
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00005013);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00115093);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x0020df93);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00415f13);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x0081de93);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01025e13);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x0112dd93);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e35d13);
    assert_eq!(
        InstructionRV::Srli { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f3dc93);

}

#[test]
fn test_mcode2_ori() {
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00006013);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00116093);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x0020ef93);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00416f13);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x0081ee93);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01026e13);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x0112ed93);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e36d13);
    assert_eq!(
        InstructionRV::Ori { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f3ec93);

}

#[test]
fn test_mcode2_andi() {
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x00007013);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x00117093);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x0020ff93);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x00417f13);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x0081fe93);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x01027e13);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x0112fd93);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x01e37d13);
    assert_eq!(
        InstructionRV::Andi { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x01f3fc93);

}


#[test]
fn test_mcode2_srai() {
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X0, rs1: Reg::X0, imm: 0 }.to_mcode(),
        0x40005013);
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X1, rs1: Reg::X2, imm: 1 }.to_mcode(),
        0x40115093);
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X31, rs1: Reg::X1, imm: 2 }.to_mcode(),
        0x4020df93);
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X30, rs1: Reg::X2, imm: 4 }.to_mcode(),
        0x40415f13); 
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X29, rs1: Reg::X3, imm: 8 }.to_mcode(),
        0x4081de93); 
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X28, rs1: Reg::X4, imm: 16 }.to_mcode(),
        0x41025e13);
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X27, rs1: Reg::X5, imm: 17 }.to_mcode(),
        0x4112dd93);
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X26, rs1: Reg::X6, imm: 30 }.to_mcode(),
        0x41e35d13);
    assert_eq!(
        InstructionRV::Srai { rd: Reg::X25, rs1: Reg::X7, imm: 31 }.to_mcode(),
        0x41f3dc93);
   
}

#[test]
fn test_mcode2_lb() {
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_mcode(),
        0x00008003);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X1, offs: 1, rs1: Reg::X2 }.to_mcode(),
        0x00110083);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X2, offs: 2, rs1: Reg::X3 }.to_mcode(),
        0x00218103);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X4, offs: 4, rs1: Reg::X4 }.to_mcode(),
        0x00420203);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X5, offs: 8, rs1: Reg::X5 }.to_mcode(),
        0x00828283);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X6, offs: -1, rs1: Reg::X6 }.to_mcode(),
        0xfff30303);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X7, offs: -2048, rs1: Reg::X7 }.to_mcode(),
        0x80038383);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X8, offs: -2, rs1: Reg::X8 }.to_mcode(),
        0xffe40403);
    assert_eq!(
        InstructionRV::Lb { rd: Reg::X9, offs: 2047, rs1: Reg::X9 }.to_mcode(),
        0x7ff48483);
}

#[test]
fn test_mcode2_lh() {
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_mcode(), 
        0x00009003);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_mcode(),
        0x00111083);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_mcode(),
        0x00219103);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_mcode(),
        0x00421203);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_mcode(),
        0x00829283);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_mcode(),
        0xfff31303);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_mcode(),
        0x80039383);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_mcode(),
        0xffe41403);
    assert_eq!(
        InstructionRV::Lh{ rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_mcode(),
        0x7ff49483); 
}

#[test]
fn test_mcode2_lw() {
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_mcode(), 
        0x0000a003);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_mcode(), 
        0x00112083);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_mcode(),
        0x0021a103);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_mcode(),
        0x00422203);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_mcode(),
        0x0082a283);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_mcode(),
        0xfff32303);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_mcode(),
        0x8003a383);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_mcode(),
        0xffe42403);
    assert_eq!(
        InstructionRV::Lw{ rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_mcode(),
        0x7ff4a483);
}

#[test]
fn test_mcode2_ld() {
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X0, offs: 0, rs1: Reg::X1}.to_mcode(), 
        0x0000b003);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X1, offs: 1, rs1: Reg::X2}.to_mcode(), 
        0x00113083);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X2, offs: 2, rs1: Reg::X3}.to_mcode(), 
        0x0021b103);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X4, offs: 4, rs1: Reg::X4}.to_mcode(), 
        0x00423203);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X5, offs: 8, rs1: Reg::X5}.to_mcode(), 
        0x0082b283);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X6, offs: -1, rs1: Reg::X6}.to_mcode(), 
        0xfff33303);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X7, offs: -2048, rs1: Reg::X7}.to_mcode(), 
        0x8003b383);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X8, offs: -2, rs1: Reg::X8}.to_mcode(), 
        0xffe43403);
    assert_eq!(
        InstructionRV::Ld{rd: Reg::X9, offs: 2047, rs1: Reg::X9}.to_mcode(), 
        0x7ff4b483);
}