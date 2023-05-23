use crate::misc::approx_equal;

const N: usize = 2;
type Row = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    rows: [Row; N],
}

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }

    pub fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.rows[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.rows[row][col]
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.rows
            .iter()
            .zip(other.rows.iter())
            .all(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .all(|(a, b)| approx_equal(*a, *b))
            })
    }
}