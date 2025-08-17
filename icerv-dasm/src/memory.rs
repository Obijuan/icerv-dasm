pub const TEST:u8 = 0xff;

//────────────────────────────────────────────────
//  MODELADO DE LA MEMORIA
//──────────────────────────────────────────────── 
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    //────────────────────────────────────────────────
    //  Constructor
    //──────────────────────────────────────────────── 
    pub fn new(data: Vec<u8>) -> Self {
        Memory { data }
    }

    //────────────────────────────────────────────────
    //  Leer un byte
    //──────────────────────────────────────────────── 
    pub fn read_byte(&self, addr: u32) -> u8 {
        self.data[addr as usize]
    }
}

#[test]
fn test_mem() {
    println!("holi!!");
}
