use super::{digit::Digit, GRID_SIZE};

#[derive(Debug)]
pub struct Candidates {
    pub data: [Digit; GRID_SIZE],
    length: usize,
}

impl Candidates {
    pub fn new() -> Self {
        Self {
            data: [0; GRID_SIZE].map(|zero| Digit::new(zero)),
            length: 0,
        }
    }

    pub fn add(&mut self, digit: Digit) {
        self.data[self.length] = digit;
        self.length += 1;
    }
}
