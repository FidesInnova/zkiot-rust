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
use rustnomial::Degree;
use rustnomial::SizedPolynomial;
use rustnomial::Term;

use crate::math::Mfp;
use crate::math::Poly;
use crate::to_bint;

/// Generates a vector of Mfp values based on the setup parameters and a random number
pub fn setup(max: u64, tau: u64, p: u64, g: u64) -> Vec<u64> {
    // Random number 
    let tau = tau % (p - 1);
    let mut tmp = g % p;

    (0..max)
        .map(|_| {
            let current = tmp;
            tmp = ((current as u128 * tau as u128) % p as u128) as u64;
            current
        })
        .collect()
}


/// Computes the commitment of a polynomial using the provided commitment keys
pub fn commit(poly_in: &Poly, ck: &[Mfp]) -> Mfp {
    let mut res_poly = Mfp::ZERO;

    if let Degree::Num(deg) = poly_in.degree() {

        // Ensure that the number of commitment keys is greater than the polynomial degree
        assert!(ck.len() > deg, "Error: The number of commitment keys ({}), must be greater than the polynomial degree ({}).", ck.len(), deg);

        for i in 0..=deg {
            match poly_in.term_with_degree(i) {
                Term::ZeroTerm => {
                    continue;
                }
                Term::Term(t, _) => {
                    let mul = Mfp::from(to_bint!(t) as u128 * to_bint!(ck[i]) as u128);
                    res_poly += mul;
                }
            }
        }
    }

    res_poly
}




#[cfg(test)]
mod test_kzg {
    use crate::math::P;

    use super::*;

    #[test]
    fn test_setup() {
        let max = 5;
        let tau = 119;
        let g = 2;

        let result = setup(max, tau, 181, g);

        assert_eq!(result.len(), max as usize);
        
        assert_eq!(result, vec![2, 57, 86, 98, 78]);
    }

    #[test]
    fn test_commit() {
        let poly = Poly::new(vec![
            Mfp::from(234),
            Mfp::from(12),
            Mfp::ZERO,
            Mfp::from(99)
        ]);
        let ck = vec![Mfp::from(22), Mfp::from(180), Mfp::from(571), Mfp::from(174), Mfp::from(333)];

        let result = commit(&poly, &ck);
        
        assert_eq!(result, Mfp::from(152));
    }
}