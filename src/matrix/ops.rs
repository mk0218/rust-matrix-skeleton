use std::ops::{Add, Mul, Sub};

use super::*;

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl Mul for Matrix {
    type Output = Self; 

    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Matrix {
    fn scalar_mul(self, scalar: i32) -> Self {
        unimplemented!();
    }

    fn transpose(self) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod test_ops {
    use crate::m;
    use super::*;
    
    #[test]
    fn add() {
        assert_eq!(m![3; 3, 2], m![1; 3, 2] + m![2; 3, 2]);
    }

    #[test]
    fn sub() {
        assert_eq!(m![1; 3, 2], m![3; 3, 2] - m![2; 3, 2]);
    }

    #[test]
    fn mul() {
        assert_eq!(m![6; 2, 2], m![1; 3, 2] * m![2; 2, 3]);
    }

    #[test]
    fn scalar_mul() {
        assert_eq!(m![3; 3, 2], m![1; 3, 2].scalar_mul(3));
    }

    #[test]
    fn transpose() {
        assert_eq!(m![1, 4; 2, 5; 3, 6], m![1, 2, 3; 4, 5, 6].transpose());
    }
}