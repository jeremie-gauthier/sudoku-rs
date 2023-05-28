use super::{digit::Digit, GRID_SIZE};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Candidates {
    pub data: [Digit; GRID_SIZE],
    pub length: usize,
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

impl IntoIterator for Candidates {
    type Item = Digit;
    type IntoIter = CandidatesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        CandidatesIntoIterator {
            candidates: self,
            index: 0,
        }
    }
}

pub struct CandidatesIntoIterator {
    candidates: Candidates,
    index: usize,
}

impl Iterator for CandidatesIntoIterator {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index.cmp(&self.candidates.length) {
            Ordering::Less => {
                let index = self.index;
                self.index += 1;
                Some(self.candidates.data[index])
            }
            _ => None,
        }
    }
}
