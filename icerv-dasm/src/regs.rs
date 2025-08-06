
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

    pub fn new(num: u8) -> Self {
      match num { 
          0 => Reg::X0,
          1 => Reg::X1,
          2 => Reg::X2,
          3 => Reg::X3,
          4 => Reg::X4,
          5 => Reg::X5,
          6 => Reg::X6,
          7 => Reg::X7,
          8 => Reg::X8,
          9 => Reg::X9,
          10 => Reg::X10,
          11 => Reg::X11,
          12 => Reg::X12,
          13 => Reg::X13,
          14 => Reg::X14,
          15 => Reg::X15,
          16 => Reg::X16,
          17 => Reg::X17,
          18 => Reg::X18,
          19 => Reg::X19,
          20 => Reg::X20,
          21 => Reg::X21,
          22 => Reg::X22,
          23 => Reg::X23,
          24 => Reg::X24,
          25 => Reg::X25,
          26 => Reg::X26,
          27 => Reg::X27,
          28 => Reg::X28,
          29 => Reg::X29,
          30 => Reg::X30,
          31 => Reg::X31,
          _ => panic!("Registro inválido: {}", num),
      }
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
