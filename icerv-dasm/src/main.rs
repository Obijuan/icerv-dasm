//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────

//-- Instrucciones RV32I
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

mod regs;
mod instructionrv;
mod mcode;
mod opcoderv;

//-- Registros del RISCV
use regs::Reg;
use instructionrv::InstructionRV;

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
        0x08000217, // 🟢auipc x4, 0x08000
        0xff1ff26f, // 🟢jal x4, -16
        0xfff500e7, // 🟢jalr x1, -1(x10)
        0x00000073, // 🟢ecall
        0x00100073, // 🟢ebreak
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
        let inst = InstructionRV::from_mcode(machine_code).to_string();
        //let inst: String = disassemble(insts[i]);

        //-- Imprimirla!
        println!("🟢 [{machine_code:#010X}]: {inst}");

    }
}

fn main_test1() {

    println!("\n------ TESTING 1.....");
    let inst = [

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
        InstructionRV::Auipc {rd: Reg::X4, imm: 0x08000},

        //-- Instrucciones tipo J
        InstructionRV::Jal {rd: Reg::X4, offs: -16},
        InstructionRV::Jalr {rd: Reg::X1, offs: -1, rs1: Reg::X10},

        //-- Instrucciones tipo ecall
        InstructionRV::Ecall,
        InstructionRV::Ebreak,
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
        0xfff500e7, //-- jalr x1, -1(x10)
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

#[test]
fn test1() {
    //-- Instruciones Tipo I
  assert_eq!(
    InstructionRV::Addi{rd: Reg::X1, rs1: Reg::X0, imm: 1}.to_string(),
    "addi x1, x0, 1");
  assert_eq!(
    InstructionRV::Slli{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "slli x1, x2, 1");
  assert_eq!(
    InstructionRV::Slti{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "slti x1, x2, 1");
  assert_eq!(
    InstructionRV::Sltiu{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "sltiu x1, x2, 1");
  assert_eq!(
    InstructionRV::Xori{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "xori x1, x2, 1");
  assert_eq!(
    InstructionRV::Srli{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "srli x1, x2, 1");
  assert_eq!(
    InstructionRV::Ori{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "ori x1, x2, 1");
  assert_eq!(
    InstructionRV::Andi{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "andi x1, x2, 1");
  assert_eq!(
    InstructionRV::Srai{rd: Reg::X1, rs1: Reg::X2, imm: 1}.to_string(),
    "srai x1, x2, 1");
  assert_eq!(
    InstructionRV::Lb { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lb x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lh { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lh x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lw { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lw x0, 0(x1)");
  assert_eq!(
    InstructionRV::Ld { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "ld x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lbu { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lbu x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lhu { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lhu x0, 0(x1)");
  assert_eq!(
    InstructionRV::Lwu { rd: Reg::X0, offs: 0, rs1: Reg::X1 }.to_string(),
    "lwu x0, 0(x1)");

  //-- Instruciones Tipo R
  assert_eq!(
    InstructionRV::Add { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "add x0, x1, x2");
  assert_eq!(
    InstructionRV::Sub { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sub x0, x1, x2");
  assert_eq!(
    InstructionRV::Sll { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sll x0, x1, x2");
  assert_eq!(
    InstructionRV::Sltu { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sltu x0, x1, x2");
  assert_eq!(
    InstructionRV::Xor { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "xor x0, x1, x2");
  assert_eq!(
    InstructionRV::Srl { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "srl x0, x1, x2");
  assert_eq!(
    InstructionRV::Or { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "or x0, x1, x2");
  assert_eq!(
    InstructionRV::Sra { rd: Reg::X0, rs1: Reg::X1, rs2: Reg::X2}.to_string(),
    "sra x0, x1, x2");

  //-- Instruciones Tipo S
  assert_eq!(
    InstructionRV::Sb { rs2: Reg::X0, offs: 0, rs1: Reg::X1}.to_string(),
    "sb x0, 0(x1)");
  assert_eq!(
    InstructionRV::Sh { rs2: Reg::X2, offs: -1, rs1: Reg::X3}.to_string(),
    "sh x2, -1(x3)");
  assert_eq!(
    InstructionRV::Sw { rs2: Reg::X4, offs: 2047, rs1: Reg::X5}.to_string(),
    "sw x4, 2047(x5)");
  assert_eq!(
    InstructionRV::Sd { rs2: Reg::X5, offs: -2048, rs1: Reg::X6}.to_string(),
    "sd x5, -2048(x6)");

  //-- Instructiones Tipo B
  assert_eq!(
    InstructionRV::Beq {rs1: Reg::X1, rs2: Reg::X2, offs: -4}.to_string(),
    "beq x1, x2, -4");
  assert_eq!(
    InstructionRV::Bne {rs1: Reg::X3, rs2: Reg::X4, offs: -8}.to_string(),
    "bne x3, x4, -8");
  assert_eq!(
    InstructionRV::Blt {rs1: Reg::X5, rs2: Reg::X6, offs: -12}.to_string(),
    "blt x5, x6, -12");
  assert_eq!(
    InstructionRV::Bge {rs1: Reg::X7, rs2: Reg::X8, offs: 24}.to_string(),
    "bge x7, x8, 24");
  assert_eq!(
    InstructionRV::Bltu {rs1: Reg::X9, rs2: Reg::X10, offs: 20}.to_string(),
    "bltu x9, x10, 20");
  assert_eq!(
    InstructionRV::Bgeu {rs1: Reg::X11, rs2: Reg::X12, offs: 16}.to_string(),
    "bgeu x11, x12, 16");

  //-- Instrucciones Tipo U
  assert_eq!(
    InstructionRV::Lui {rd: Reg::X6, imm: 0x80000}.to_string(),
    "lui x6, 0x80000");
  assert_eq!(
    InstructionRV::Auipc {rd: Reg::X4, imm: 0x08000}.to_string(),
    "auipc x4, 0x08000");

  //-- Instrucciones tipo J
  assert_eq!(
    InstructionRV::Jal {rd: Reg::X4, offs: -16}.to_string(),
    "jal x4, -16");
  assert_eq!(
    InstructionRV::Jalr {rd: Reg::X1, offs: -1, rs1: Reg::X10}.to_string(),
    "jalr x1, -1(x10)");

  //-- Instrucciones tipo ecall
  //InstructionRV::Ecall,
  //InstructionRV::Ebreak,
}

