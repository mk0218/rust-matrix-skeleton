mod ops;
// Comment above line and uncomment below to test the example.
// mod example;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(Vec<Vec<i32>>);

type RawMatrix = Vec<Vec<i32>>;

impl From<RawMatrix> for Matrix {
    fn from(v: RawMatrix) -> Self {
        // vec![] and vec![vec![]] are not allowed as v.
        if v.len() == 0 || v[0].len() == 0 {
            panic!("Empty vector cannot be converted into a Matrix.");
        }
        
        // Check if given v is a valid matrix.
        let err = v
            .iter()
            .map(|r| r.len())
            .find(|&l| l != v[0].len());
        
        if let Some(_) = err {
            panic!("The length of rows are not regular.");
        }

        Matrix(v)
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

    [$($($v:expr),+);+$(;)?] => {
        Matrix::from(vec![$(vec![$($v),+]),+])
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
    
    #[test]
    #[ignore]
    #[should_panic(expected = "Empty")]
    fn test_empty_vector_1() {
        let _: Matrix = Matrix::from(vec![]);
    }
    
    #[test]
    #[ignore]
    #[should_panic(expected = "Empty")]
    fn test_empty_vector_2() {
        let _ = Matrix::from(vec![vec![]]);
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "not regular")]
    fn test_not_a_matrix() {
        let _ = Matrix::from(vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2],
        ]);
    }
    
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
        assert_eq!(expected, m![1, 2, 3; 4, 5, 6;]);
    }
}