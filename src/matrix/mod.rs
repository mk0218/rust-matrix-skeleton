mod ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    pub fn nrows(&self) -> usize {
        self.0.len()
    }

    pub fn ncols(&self) -> usize {
        self.0.get(0).expect(
            "A matrix should have at least one item."
        ).len()
    }
}

type RawMatrix = Vec<Vec<i32>>;

impl From<RawMatrix> for Matrix {
    fn from(value: RawMatrix) -> Self {
        Matrix(value)
    }
}

impl Into<RawMatrix> for Matrix {
    fn into(self) -> RawMatrix {
        self.0
    }
}

#[macro_export]
macro_rules! m {
    [$v:expr; $r:expr, $c: expr] => {
        Matrix::from(vec![vec![$v; $c]; $r])
    };

    [$($($v:expr),+);+] => {
        Matrix::from(vec![$(vec![$($v),+]),+])
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
    
    #[test]
    #[ignore]
    fn test_macro_1() {
        let expected = Matrix(vec![vec![1, 1], vec![1, 1], vec![1, 1]]);
        assert_eq!(expected, m![1; 3, 2])
    }

    #[test]
    #[ignore]
    fn test_macro_2() {
        let expected = Matrix(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(expected, m![1, 2, 3; 4, 5, 6]);
    }
}