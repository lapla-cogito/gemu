#[derive(Clone)]
pub struct Hram(
    Box<[u8; 0x80]>, // HRAM has 128 Bytes
);

impl Hram {
    pub fn new() -> Self {
        Self(Box::new([0; 0x80]))
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x7f]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.0[(addr as usize) & 0x7f] = data;
    }
}
