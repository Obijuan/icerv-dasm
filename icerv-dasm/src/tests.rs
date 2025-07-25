use crate::disassemble;

#[test]
fn test_disassemble_ecall_ebreak() {
    assert_eq!(disassemble(0x00000073), "ecall");
    assert_eq!(disassemble(0x00100073), "ebreak");
}

#[test]
fn test_disassemble_jal() {
    assert_eq!(disassemble(0x0000006f), "jal x0, 0");
    assert_eq!(disassemble(0xffdff0ef), "jal x1, -4");
    assert_eq!(disassemble(0xff9ff16f), "jal x2, -8");
    assert_eq!(disassemble(0xff5ff1ef), "jal x3, -12");
    assert_eq!(disassemble(0xff1ff26f), "jal x4, -16");
    assert_eq!(disassemble(0x014002ef), "jal x5, 20");
    assert_eq!(disassemble(0x0100036f), "jal x6, 16");
    assert_eq!(disassemble(0x00c003ef), "jal x7, 12");
    assert_eq!(disassemble(0x0080046f), "jal x8, 8");
    assert_eq!(disassemble(0x004004ef), "jal x9, 4");
}

#[test]
fn test_disassemble_jalr() {
    assert_eq!(disassemble(0x00000067), "jalr x0, 0(x0)");
    assert_eq!(disassemble(0xfff500e7), "jalr x1, -1(x10)");
    assert_eq!(disassemble(0xffe58167), "jalr x2, -2(x11)");
    assert_eq!(disassemble(0xffc601e7), "jalr x3, -4(x12)");
    assert_eq!(disassemble(0xff868267), "jalr x4, -8(x13)");
    assert_eq!(disassemble(0x800702e7), "jalr x5, -2048(x14)");
    assert_eq!(disassemble(0x7ff78367), "jalr x6, 2047(x15)");
    assert_eq!(disassemble(0x010803e7), "jalr x7, 16(x16)");
    assert_eq!(disassemble(0x00888467), "jalr x8, 8(x17)");
    assert_eq!(disassemble(0x004904e7), "jalr x9, 4(x18)");
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

