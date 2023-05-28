use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Digit(u8);

impl Digit {
    pub fn new(digit: u8) -> Self {
        Self(digit)
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
