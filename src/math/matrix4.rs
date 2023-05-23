use std::ops::Mul;

use super::matrix3::Matrix3;
use super::tuple::Tuple;
use crate::misc::{self, approx_equal};

const N: usize = 4;
type Row = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    rows: [Row; N],
}

impl Matrix4 {
    pub fn from_rows(rows: [Row; N]) -> Self {
        Self { rows }
    }

    pub fn identity() -> Self {
        let mut zeroes = Self::zeroes();

        (0..N).for_each(|index| {
            *zeroes.get_mut(index, index) = 1.;
        });

        zeroes
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::zeroes();

        for row in 0..N {
            for col in 0..N {
                *result.get_mut(col, row) = self.get(row, col);
            }
        }

        result
    }

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

    pub fn submatrix(&self, row_to_delete: usize, col_to_delete: usize) -> Matrix3 {
        let mut result = Matrix3::zeroes();

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

    fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();

        if approx_equal(det, 0.) {
            None
        } else {
            let mut result = Matrix4::zeroes();

            for row in 0..N {
                for col in 0..N {
                    let cofactor = self.cofactor(row, col);

                    *result.get_mut(col, row) = cofactor / det;
                }
            }

            Some(result)
        }
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

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::from_rows([
            [1., 0., 0., x],
            [0., 1., 0., y],
            [0., 0., 1., z],
            [0., 0., 0., 1.],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::from_rows([
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_x(angle_radians: f64) -> Self {
        let r = angle_radians;
        Self::from_rows([
            [1., 0., 0., 0.],
            [0., r.cos(), -r.sin(), 0.],
            [0., r.sin(), r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_y(angle_radians: f64) -> Self {
        let r = angle_radians;
        Self::from_rows([
            [r.cos(), 0., r.sin(), 0.],
            [0., 1., 0., 0.],
            [-r.sin(), 0., r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_z(angle_radians: f64) -> Self {
        let r = angle_radians;

        Self::from_rows([
            [r.cos(), -r.sin(), 0., 0.],
            [r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    #[allow(dead_code)]
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::from_rows([
            [1., xy, xz, 0.],
            [yx, 1., yz, 0.],
            [zx, zy, 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::from_rows([[0.; N]; N]);

        for row in 0..N {
            for col in 0..N {
                for k in 0..N {
                    *result.get_mut(row, col) += self.get(row, k) * rhs.get(k, col);
                }
            }
        }

        result
    }
}

impl Mul<f64> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let rows = self.rows;
        let new_rows = [
            [
                rows[0][0] * rhs,
                rows[0][1] * rhs,
                rows[0][2] * rhs,
                rows[0][3] * rhs,
            ],
            [
                rows[1][0] * rhs,
                rows[1][1] * rhs,
                rows[1][2] * rhs,
                rows[1][3] * rhs,
            ],
            [
                rows[2][0] * rhs,
                rows[2][1] * rhs,
                rows[2][2] * rhs,
                rows[2][3] * rhs,
            ],
            [
                rows[3][0] * rhs,
                rows[3][1] * rhs,
                rows[3][2] * rhs,
                rows[3][3] * rhs,
            ],
        ];

        Self { rows: new_rows }
    }
}
impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Self::Output {
        Tuple::new(
            row_to_tuple(self.rows[0]).dot(tuple),
            row_to_tuple(self.rows[1]).dot(tuple),
            row_to_tuple(self.rows[2]).dot(tuple),
            row_to_tuple(self.rows[3]).dot(tuple),
        )
    }
}

fn row_to_tuple(row: Row) -> Tuple {
    Tuple::new(row[0], row[1], row[2], row[3])
}

impl PartialEq for Matrix4 {
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


