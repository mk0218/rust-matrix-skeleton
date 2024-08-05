use std::ops::{Add, Mul, Sub};

use super::*;

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl Mul for Matrix {
    type Output = Self; 

    fn mul(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl Matrix {
    pub fn scalar_mul(self, scalar: i32) -> Self {
        todo!();
    }

    pub fn transpose(self) -> Self {
        todo!();
    }
}
