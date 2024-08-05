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
    #[should_panic]
    fn add_panic() {
        let _ = m![1; 3, 2] + m![2; 2, 2];
    }

    #[test]
    fn sub() {
        assert_eq!(m![1; 3, 2], m![3; 3, 2] - m![2; 3, 2]);
    }

    #[test]
    #[should_panic]
    fn sub_panic() {
        let _ = m![1; 2, 2] + m![0; 3, 3];
    }

    #[test]
    fn mul() {
        assert_eq!(m![4; 3, 3], m![1; 3, 2] * m![2; 2, 3]);
    }

    #[test]
    #[should_panic]
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