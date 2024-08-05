use std::{iter::Zip, ops::{Add, Mul, Sub}, vec::IntoIter};

use super::*;

fn zip<T>(v1: Vec<T>, v2: Vec<T>) -> Zip<IntoIter<T>, IntoIter<T>> {
    Iterator::zip(v1.into_iter(), v2.into_iter())
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.nrows() != rhs.nrows() || self.ncols() != rhs.ncols() {
            panic!("rhs must have same number of rows and columns.")
        }
        zip(self.0, rhs.0).map(|(r1, r2)| {
            zip(r1, r2).map(|(v1, v2)| v1 + v2).collect()
        }).collect::<Vec<_>>().into()
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.scalar_mul(-1)
    }
}

impl Mul for Matrix {
    type Output = Self; 

    fn mul(self, rhs: Self) -> Self::Output {
        if self.ncols() != rhs.nrows() {
            panic!(
                "Number of columns of lhs and number of columns of rhs must be the same."
            );
        }

        (0..self.nrows()).map(|j| {
            (0..rhs.ncols()).map(|i| {
                (0..self.ncols()).fold(0, |acc, k| {
                    acc + self.0[j][k] * rhs.0[k][i]
                })
            }).collect()
        }).collect::<Vec<_>>().into()
    }
}

impl Matrix {
    pub fn scalar_mul(self, scalar: i32) -> Self {
        self.0.into_iter().map(|r| {
            r.into_iter().map(|v| v * scalar).collect()
        }).collect::<Vec<_>>().into()
    }

    pub fn transpose(self) -> Self {
        (0..self.ncols()).map(|i| {
            (0..self.nrows()).map(|j| self.0[j][i]).collect()
        }).collect::<Vec<_>>().into()
    }
}
