#[derive(Debug, Clone)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}
