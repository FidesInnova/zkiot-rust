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
use crate::math::P;
use crate::to_bint;

pub fn setup(max: u64, tau: u64, g: u64) -> Vec<Mfp> {
    let tau = tau % (P - 1);
    let mut tmp = Mfp::from(g);

    (0..max)
        .map(|_| {
            let current = tmp;
            tmp = Mfp::from(to_bint!(current) * tau);
            current
        })
        .collect()
}

pub fn commit(poly_in: &Poly, ck: &[Mfp]) -> Mfp {
    let mut res_poly = Mfp::ZERO;

    if let Degree::Num(deg) = poly_in.degree() {
        for i in 0..=deg {
            match poly_in.term_with_degree(i) {
                Term::ZeroTerm => {
                    continue;
                }
                Term::Term(t, _) => {
                    let exp = Mfp::from(to_bint!(t) * to_bint!(ck[i]));
                    res_poly += exp;
                }
            }
        }
    }

    res_poly
}