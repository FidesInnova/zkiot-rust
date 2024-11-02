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

use std::{fs::File, io::BufWriter, path::PathBuf};

use anyhow::Result;
use ark_ff::Field;
use nalgebra::DMatrix;
use rand::{thread_rng, Rng};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    ahp::setup::SetupJson, json_file::{open_file, ClassData}, math::Mfp, parser::DeviceConfigJson, to_bint
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
    fn to_sparse_coordinate_form(matrix: &DMatrix<Mfp>) -> Vec<(usize, usize, u64)> {
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
    fn to_sparse_column_indices(matrix: &DMatrix<Mfp>, number_t_zeros: usize) -> Vec<u64> {
        assert!(matrix.ncols() == matrix.nrows());
        assert!(number_t_zeros <= matrix.ncols());

        let mat_size = matrix.ncols();

        let mut sp_mat = vec![];

        for i in 0..mat_size {
            for j in 0..mat_size {
                assert!(matrix[(i, j)] == Mfp::ZERO || matrix[(i, j)] == Mfp::ONE);
                if matrix[(i, j)] == Mfp::ONE {
                    // FIXME: use insert or push?
                    // sp_mat.insert(i - number_t_zeros, (j + 1).try_into().unwrap());
                    sp_mat.push((j + 1).try_into().unwrap());
                }
            }
        }


        sp_mat
    }

    pub fn generate_matrix_c(size: usize, t_zero: usize) -> DMatrix<Mfp> {
        let mut c = DMatrix::<Mfp>::zeros(size, size);

        for i in t_zero..size {
            c[(i, i)] = Mfp::ONE;
        }

        c
    }

    /// Store in Json file
    pub fn store(&self, path: &str, setup: SetupJson, class_data: &ClassData) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let matrices_json = ProgramParamsJson::new(&self.a, &self.b, setup, class_data);
        serde_json::to_writer(writer, &matrices_json)?;
        Ok(())
    }

    /// Restore Commitment from Json file
    pub fn restore(path: &str) -> Result<ProgramParamsJson> {
        let reader = open_file(&PathBuf::from(path))?;
        let matrices_json: ProgramParamsJson = serde_json::from_reader(reader)?;
        Ok(matrices_json)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProgramParamsJson {
    /// [t_zeros, ... ,col1, col2, col3, ...]
    a: Vec<u64>,

    /// [(row1, col1, val1), (row2, col2, val2), (row3, col3, val3), ...]
    b: Vec<(usize, usize, u64)>,

    #[serde(flatten)]
    setup: SetupJson,
}

impl ProgramParamsJson {
    fn new(a: &DMatrix<Mfp>, b: &DMatrix<Mfp>, setup: SetupJson, class_data: &ClassData) -> Self {
        Self {
            a: Matrices::to_sparse_column_indices(&a, matrix_t_zeros(class_data)),
            b: Matrices::to_sparse_coordinate_form(&b),
            setup
        }
    }

    pub fn get_matrix_a(&self, size: usize, number_t_zeros: usize) -> DMatrix<Mfp> {
        let mut mat_a = DMatrix::<Mfp>::zeros(size, size);

        for (i, &j) in self.a.iter().enumerate() {
            if j == 0 {
                continue;
            }
            mat_a[(i + number_t_zeros, (j - 1) as usize)] = Mfp::ONE;
        }

        mat_a
    }

    pub fn get_matrix_b(&self, size: usize) -> DMatrix<Mfp> {
        let mut mat_b = DMatrix::<Mfp>::zeros(size, size);

        for &(i, j, val) in self.b.iter() {
            mat_b[(i, j)] = Mfp::from(val);
        }

        mat_b
    }
}

/// Returns the size of the matrix based on class data
pub fn matrix_size(class_data: &ClassData) -> usize {
    (class_data.n_g + class_data.n_i + 1).try_into().unwrap()
}

/// Returns the number of zero rows in the matrix based on class data
pub fn matrix_t_zeros(class_data: &ClassData) -> usize {
    // Number of rows (|x| = numebr_t_zero, where numebr_t_zero = ni + 1)
    (class_data.n_i + 1).try_into().unwrap()
}
