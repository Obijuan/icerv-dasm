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





