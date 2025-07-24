#[derive(Clone, Copy)] 
pub enum Reg {
//────────────────────────────────────────────────
//  Registros del RISC-V
//────────────────────────────────────────────────
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}    

impl Reg {
    
    pub fn to_str(&self) -> String {
    //────────────────────────────────────────────────
    //  Convertir el registro a una cadena
    //    Ej. Reg::X1 -> "x1"
    //────────────────────────────────────────────────

        format!("x{}", *self as u8)
    }
}

#[test]
fn test_regs() {
    //────────────────────────────────────────────────
    //  Test de los registros
    //────────────────────────────────────────────────
    assert_eq!(Reg::X0.to_str(), "x0");
    assert_eq!(Reg::X1.to_str(), "x1");
    assert_eq!(Reg::X2.to_str(), "x2");
    assert_eq!(Reg::X3.to_str(), "x3");
    assert_eq!(Reg::X4.to_str(), "x4");
    assert_eq!(Reg::X5.to_str(), "x5");
    assert_eq!(Reg::X6.to_str(), "x6");
    assert_eq!(Reg::X7.to_str(), "x7");
    assert_eq!(Reg::X8.to_str(), "x8");
    assert_eq!(Reg::X9.to_str(), "x9");
    assert_eq!(Reg::X10.to_str(), "x10");
    assert_eq!(Reg::X11.to_str(), "x11");
    assert_eq!(Reg::X12.to_str(), "x12");
    assert_eq!(Reg::X13.to_str(), "x13");
    assert_eq!(Reg::X14.to_str(), "x14");
    assert_eq!(Reg::X15.to_str(), "x15");
    assert_eq!(Reg::X16.to_str(), "x16");
    assert_eq!(Reg::X17.to_str(), "x17");
    assert_eq!(Reg::X18.to_str(), "x18");
    assert_eq!(Reg::X19.to_str(), "x19");
    assert_eq!(Reg::X20.to_str(), "x20");
    assert_eq!(Reg::X21.to_str(), "x21");
    assert_eq!(Reg::X22.to_str(), "x22");
    assert_eq!(Reg::X23.to_str(), "x23");
    assert_eq!(Reg::X24.to_str(), "x24");
    assert_eq!(Reg::X25.to_str(), "x25");
    assert_eq!(Reg::X26.to_str(), "x26");
    assert_eq!(Reg::X27.to_str(), "x27");
    assert_eq!(Reg::X28.to_str(), "x28");
    assert_eq!(Reg::X29.to_str(), "x29");
    assert_eq!(Reg::X30.to_str(), "x30");
    assert_eq!(Reg::X31.to_str(), "x31");   
}   
