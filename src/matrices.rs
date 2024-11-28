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

use ark_ff::Field;
use nalgebra::DMatrix;

use crate::{
    math::Mfp, to_bint
};

#[derive(Debug, Clone)]
/// A struct representing a collection of matrices used in computations.
pub struct Matrices {
    pub a: DMatrix<Mfp>,
    pub b: DMatrix<Mfp>,
    pub c: DMatrix<Mfp>,
    // pub z: DMatrix<Mfp>,
    pub size: usize,
}

impl Matrices {
    pub fn new(size: usize) -> Self {
        let a = DMatrix::<Mfp>::zeros(size, size);
        let b = DMatrix::<Mfp>::zeros(size, size);
        let c = DMatrix::<Mfp>::zeros(size, size);
        // let z = DMatrix::<Mfp>::zeros(size, 1);

        Self { a, b, c, size }
    }

    /// Converts a dense matrix to a sparse coordinate form, represented as a vector of
    /// tuples (i, j, val), where `i` is the row index, `j` is the column index, and
    /// `val` is the value at that position in the matrix.
    pub fn to_sparse_coordinate_form(matrix: &DMatrix<Mfp>) -> Vec<(usize, usize, u64)> {
        let mut sp_mat = vec![];

        for i in 0..matrix.ncols() {
            for j in 0..matrix.nrows() {
                if matrix[(i, j)] != Mfp::ZERO {
                    sp_mat.push((i, j, to_bint!(matrix[(i, j)])));
                }
            }
        }

        sp_mat
    }

    /// Converts a dense matrix to a sparse representation by storing the column indices
    /// where the values are set to 1. The resulting vector contains the column indices
    /// for each row, with the assumption that all other values are 0.
    pub fn to_sparse_column_indices(matrix: &DMatrix<Mfp>, number_t_zeros: usize) -> Vec<u64> {
        assert!(matrix.ncols() == matrix.nrows());
        assert!(number_t_zeros <= matrix.ncols());

        let mat_size = matrix.ncols();

        let mut sp_mat = vec![];

        for i in 0..mat_size {
            for j in 0..mat_size {
                assert!(matrix[(i, j)] == Mfp::ZERO || matrix[(i, j)] == Mfp::ONE);
                if matrix[(i, j)] == Mfp::ONE {
                    sp_mat.push((j).try_into().unwrap());
                }
            }
        }


        sp_mat
    }

    /// Generates a square matrix of specified size with ones on the diagonal starting from the t_zero index
    pub fn generate_matrix_c(size: usize, t_zero: usize) -> DMatrix<Mfp> {
        let mut c = DMatrix::<Mfp>::zeros(size, size);

        for i in t_zero..size {
            c[(i, i)] = Mfp::ONE;
        }

        c
    }
}
