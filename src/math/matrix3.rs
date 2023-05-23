use super::matrix2::Matrix2;
use crate::misc::{self, approx_equal};

const N: usize = 3;
type Row = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    rows: [Row; N],
}

impl Matrix3 {
    pub fn determinant(&self) -> f64 {
        (0..N)
            .map(|col| {
                let element = self.get(0, col);

                element * self.cofactor(0, col)
            })
            .sum()
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.rows[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.rows[row][col]
    }

    pub fn submatrix(&self, row_to_delete: usize, col_to_delete: usize) -> Matrix2 {
        let mut result = Matrix2::zeroes();

        for row in 0..N {
            for col in 0..N {
                if let Some((offset_row, offset_col)) =
                    misc::cmp_to_offset(row.cmp(&row_to_delete), col.cmp(&col_to_delete))
                {
                    let actual_row = (row as i32 + offset_row) as usize;
                    let actual_col = (col as i32 + offset_col) as usize;

                    *result.get_mut(actual_row, actual_col) = self.get(row, col);
                }
            }
        }
        result
    }

    pub fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    fn minor(&self, row_to_delete: usize, col_to_delete: usize) -> f64 {
        self.submatrix(row_to_delete, col_to_delete).determinant()
    }

    fn cofactor(&self, row_to_delete: usize, col_to_delete: usize) -> f64 {
        let row_sign = if row_to_delete % 2 == 0 { 1 } else { -1 };
        let col_sign = if col_to_delete % 2 == 0 { 1 } else { -1 };
        let sign = row_sign * col_sign;

        sign as f64 * self.minor(row_to_delete, col_to_delete)
    }
}

impl PartialEq for Matrix3 {
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


