mod ops;

#[derive(Debug, PartialEq)]
pub struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    pub fn from(v: Vec<Vec<i32>>) -> Self {
        Matrix(v)
    }
}

#[macro_export]
macro_rules! m {
    [$v:expr; $c:expr, $r: expr] => {
        Matrix(vec![vec![$v; $c]; $r])
    };

    [$($($v:expr),+);+] => {
        Matrix(vec![$(vec![$($v),+]),+])
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
    
    #[test]
    #[ignore]
    fn test_macro_1() {
        let expected = Matrix(vec![vec![1, 1, 1], vec![1, 1, 1]]);
        assert_eq!(expected, m![1; 3, 2])
    }

    #[test]
    #[ignore]
    fn test_macro_2() {
        let expected = Matrix(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(expected, m![1, 2, 3; 4, 5, 6]);
    }
}