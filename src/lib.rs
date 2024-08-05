//! ## Usage
//! 
//! ### Create
//! 
//! ```rust
//! use matrix::*;
//! 
//! // Convert from Vec<Vec<i32>>
//! let m1: Matrix = vec![
//!     vec![0, 0, 0],
//!     vec![0, 0, 0]
//! ].into();
//! 
//! // Macro 1
//! let m2 = m![0; 2, 3];
//! 
//! // Macro 2
//! let m3 = m![
//!     0, 0, 0;
//!     0, 0, 0;
//! ];
//! 
//! assert_eq!(m1, m2);
//! assert_eq!(m1, m3);
//! ```
//! 
//! ### Operations (should be implemented)
//! 
//! ```rust
//! use matrix::*;
//! 
//! // Add
//! let m = m![1, 2, 3; 4, 5, 6] + m![2, 2, 2; 2, 2, 2];
//! assert_eq!(m, m![3, 4, 5; 6, 7, 8]);
//! 
//! // Sub
//! let m = m![9, 9; 9, 9] - m![1, 2; 3, 4];
//! assert_eq!(m, m![8, 7; 6, 5]);
//! 
//! // Scalar multiplication
//! let m = m![1, 2; 3, 4].scalar_mul(3);
//! assert_eq!(m, m![3, 6; 9, 12]);
//! 
//! // Multiplication
//! let m = m![1, 2, 3; 4, 5, 6] * m![2, 2, 2; 2, 2, 2; 2, 2, 2];
//! assert_eq!(m, m![12, 12, 12; 30, 30, 30]);
//! 
//! // Transpose
//! let m = m![1, 2, 3; 4, 5, 6].transpose();
//! assert_eq!(m, m![1, 4; 2, 5; 3, 6]);
//! ```

mod matrix;

pub use matrix::*;

#[cfg(test)]
mod test_ops {
    use super::*;
    
    #[test]
    fn add() {
        assert_eq!(m![3; 3, 2], m![1; 3, 2] + m![2; 3, 2]);
    }

    #[test]
    #[should_panic(expected = "rhs must have same number of rows and columns.")]
    fn add_panic() {
        let _ = m![1; 3, 2] + m![2; 2, 2];
    }

    #[test]
    fn sub() {
        assert_eq!(m![1; 3, 2], m![3; 3, 2] - m![2; 3, 2]);
    }

    #[test]
    #[should_panic(expected = "rhs must have same number of rows and columns.")]
    fn sub_panic() {
        let _ = m![1; 2, 2] + m![0; 3, 3];
    }

    #[test]
    fn mul() {
        assert_eq!(m![4; 3, 3], m![1; 3, 2] * m![2; 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Number of columns of lhs and number of columns of rhs must be the same.")]
    fn mul_panic() {
        let _ = m![1; 2, 3] * m![1; 1, 3];
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