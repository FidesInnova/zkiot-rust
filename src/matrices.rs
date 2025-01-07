// Copyright 2024 Fidesinnova, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.



#[derive(Debug, Clone, PartialEq)]
pub struct FMatrix {
    pub data: Vec<Vec<u64>>,
}

impl FMatrix {
    pub fn new(data: Vec<Vec<u64>>) -> Self {
        Self { data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0; cols]; rows];
        Self { data }
    }

    pub fn size(&self) -> usize {
        let size = self.data.len();
        assert_eq!(self.data[0].len(), size);
        size
    }
}

impl std::ops::Index<(usize, usize)> for FMatrix {
    type Output = u64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[row][col]
    }
}

impl std::ops::IndexMut<(usize, usize)> for FMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.data[row][col]
    }
}

impl std::fmt::Display for FMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size() {
            for j in 0..self.size() {
                write!(f, "{}\t", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub mod matrix_fmath {
    use crate::field::fmath;

    use super::FMatrix;

    pub fn add(a: &FMatrix, b: &FMatrix, p: u64) -> FMatrix {
        let size = a.data.len();
        assert_eq!(
            size,
            b.data.len(),
            "Matrices must have the same size for multiplication"
        );

        let mut result = FMatrix::zeros(size, size);

        for i in 0..size {
            for j in 0..size {
                result[(i, j)] = fmath::add(a[(i, j)], b[(i, j)], p);
            }
        }

        result
    }

    pub fn mul(a: &FMatrix, b: &FMatrix, p: u64) -> FMatrix {
        let size = a.data.len();
        assert_eq!(
            size,
            b.data.len(),
            "Matrices must have the same size for multiplication"
        );

        let mut result = FMatrix::zeros(size, size);

        for i in 0..size {
            for j in 0..size {
                let mut sum = 0;
                for k in 0..size {
                    let mul_num = fmath::mul(a[(i, k)], b[(k, j)], p);
                    sum = fmath::add(sum, mul_num, p);
                }
                result[(i, j)] = sum;
            }
        }

        result
    }

    pub fn component_mul(a: &FMatrix, b: &FMatrix, p: u64) -> FMatrix {
        let size = a.data.len();
        assert_eq!(
            size,
            b.data.len(),
            "Matrices must have the same size for multiplication"
        );

        let mut result = FMatrix::zeros(size, size);

        for i in 0..size {
            for j in 0..size {
                result[(i, j)] = fmath::mul(a[(i, j)], b[(i, j)], p);
            }
        }

        result
    }
}


#[derive(Debug, Clone)]
/// A struct representing a collection of matrices used in computations.
pub struct Matrices {
    pub a: FMatrix,
    pub b: FMatrix,
    pub c: FMatrix,
    pub size: usize,
}

impl Matrices {
    pub fn new(size: usize) -> Self {
        let a = FMatrix::zeros(size, size);
        let b = FMatrix::zeros(size, size);
        let c = FMatrix::zeros(size, size);

        Self { a, b, c, size }
    }

    /// Converts a dense matrix to a sparse coordinate form, represented as a vector of
    /// tuples (i, j, val), where `i` is the row index, `j` is the column index, and
    /// `val` is the value at that position in the matrix.
    pub fn to_sparse_coordinate_form(matrix: &FMatrix) -> Vec<(usize, usize, u64)> {
        let mut sp_mat = vec![];

        for i in 0..matrix.size() {
            for j in 0..matrix.size() {
                if matrix[(i, j)] != 0 {
                    sp_mat.push((i, j, matrix[(i, j)]));
                }
            }
        }

        sp_mat
    }

    /// Converts a dense matrix to a sparse representation by storing the column indices
    /// where the values are set to 1. The resulting vector contains the column indices
    /// for each row, with the assumption that all other values are 0.
    pub fn to_sparse_column_indices(matrix: &FMatrix, number_t_zeros: usize) -> Vec<u64> {
        assert!(number_t_zeros <= matrix.size());

        let mat_size = matrix.size();

        let mut sp_mat = vec![];

        for i in 0..mat_size {
            for j in 0..mat_size {
                assert!(matrix[(i, j)] == 0 || matrix[(i, j)] == 1);
                if matrix[(i, j)] == 1 {
                    sp_mat.push((j).try_into().unwrap());
                }
            }
        }


        sp_mat
    }

    /// Generates a square matrix of specified size with ones on the diagonal starting from the t_zero index
    pub fn generate_matrix_c(size: usize, t_zero: usize) -> FMatrix {
        let mut c = FMatrix::zeros(size, size);

        for i in t_zero..size {
            c[(i, i)] = 1;
        }

        c
    }
}



#[cfg(test)]
mod test_matrix_oprations {
    use super::*;

    #[test]
    fn test_add() {
        let a = FMatrix::new(vec![vec![1, 2], vec![3, 4]]);

        let b = FMatrix::new(vec![vec![5, 6], vec![7, 8]]);

        let p = 11;
        let result = matrix_fmath::add(&a, &b, p);

        let expected = FMatrix::new(vec![vec![6, 8], vec![10, 1]]);

        assert_eq!(result.data, expected.data);
    }

    #[test]
    fn test_mul() {
        let mut a = FMatrix::zeros(3, 3);
        for i in 0..3 {
            for j in 0..3 {
                a[(i, j)] = (3 * i + 2 * j) as u64;
            }
        }

        let mut b = FMatrix::zeros(3, 3);
        for i in 0..3 {
            for j in 0..3 {
                b[(i, j)] = (2 * i + 12 * j) as u64;
            }
        }

        let result = matrix_fmath::mul(&a, &b, 181);
        
        let expected = vec![
            vec![20, 92, 164],
            vec![38, 37, 36],
            vec![56, 163, 89],
        ];

        assert_eq!(result.data, expected);

    }

    #[test]
    fn test_component_mul() {
        let a = FMatrix::new(vec![vec![1, 2], vec![3, 4]]);

        let b = FMatrix::new(vec![vec![5, 6], vec![7, 8]]);

        let p = 10;
        let result = matrix_fmath::component_mul(&a, &b, p);

        let expected = FMatrix::new(vec![vec![5, 2], vec![1, 2]]);

        assert_eq!(result.data, expected.data);
    }
}
