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

use crate::{field::fmath, polynomial::FPoly};

/// Generates a vector of u64 values based on the setup parameters and a random number
pub fn setup(max: u64, tau: u64, g: u64, p: u64) -> Vec<u64> {
    // Random number
    let tau = tau % (p - 1);
    let mut tmp = g % p;

    (0..max)
        .map(|_| {
            let current = tmp;
            tmp = fmath::mul(current, tau, p);
            current
        })
        .collect()
}

/// Computes the commitment of a polynomial using the provided commitment keys
pub fn commit(poly_in: &FPoly, ck: &[u64], p: u64) -> u64 {
    let mut res_poly = 0;

    let degree = poly_in.degree();

    // Ensure that the number of commitment keys is greater than the polynomial degree
    assert!(ck.len() > degree, "Error: The number of commitment keys ({}), must be greater than the polynomial degree ({}).", ck.len(), degree);

    for i in 0..=degree {
        let term = poly_in.get_term(i);
        let mul = fmath::mul(term, ck[i], p);
        res_poly = fmath::add(res_poly, mul, p);
    }

    res_poly
}


#[cfg(test)]
mod test_kzg {
    use super::*;
    const P: u64 = 181;

    #[test]
    fn test_setup() {
        let max = 5;
        let tau = 121;
        let g = 2;

        let result = setup(max, tau, g, P);

        assert_eq!(result.len(), max as usize);
        
        assert_eq!(result, vec![2, 61, 141, 47, 76]);
    }

    #[test]
    fn test_commit() {
        let poly1 = FPoly::new(vec![
            1,
            2,
            3,
        ]);
        let ck1 = vec![3, 2, 1];
        let result = commit(&poly1, &ck1, P);
        assert_eq!(result, 14);


        let poly2 = FPoly::new(vec![
            234,
            12,
            0,
            99
        ]);
        let ck2 = vec![22, 180, 571, 174, 333];

        let result = commit(&poly2, &ck2, P);
        
        assert_eq!(result, 152);
    }
}