#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
    pub square: usize,
    ord: u8,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        let row_portion = row / 3;
        let col_portion = col / 3;
        let square = row_portion * 3 + col_portion;

        Self {
            row,
            col,
            square,
            ord: 0,
        }
    }

    pub fn set_ord(&mut self, ord: u8) {
        self.ord = ord;
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.ord == other.ord
    }
}

impl Eq for Coord {}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ord != other.ord {
            return self.ord.partial_cmp(&other.ord);
        }

        if self.row != other.row {
            return self.row.partial_cmp(&other.row);
        }

        self.col.partial_cmp(&other.col)
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ord != other.ord {
            return self.ord.cmp(&other.ord);
        }

        if self.row != other.row {
            return self.row.cmp(&other.row);
        }

        self.col.cmp(&other.col)
    }
}
