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

use poly_fmath::first_nonzero_index;

use crate::field::fmath;

#[macro_export]
macro_rules! fpoly {
    ( $( $x:expr ),* ) => {
        {
            use $crate::polynomial::FPoly;
            FPoly::new(vec![$($x,)*])
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
/// Term is a type which represents a term in a polynomial.
pub enum Term<N> {
    /// A term with coefficient zero. Has degree -inf.
    ZeroTerm,
    /// A term with non-zero coefficient and a degree.
    Term(N, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FPoly {
    pub terms: Vec<u64>,
}

impl FPoly {
    // FIXME: should i use % p here?
    pub fn new(terms: Vec<u64>) -> Self {
        let mut terms = terms;
        if terms.len() == 0 {
            terms.push(0);
        }
        Self { terms }
    }

    pub fn zero() -> Self {
        Self { terms: vec![0] }
    }

    pub fn one() -> Self {
        Self { terms: vec![1] }
    }

    pub fn one_x() -> Self {
        Self { terms: vec![1, 0] }
    }

    pub fn degree(&self) -> usize {
        let index = first_nonzero_index(&self.terms);
        if index == self.terms.len() {
            0
        } else {
            self.terms.len() - index - 1
        }
    }

    pub fn add_term(&mut self, coeff: u64, degree: usize) {
        if self.terms.len() < degree + 1 {
            let added_zeros = degree + 1 - self.terms.len();
            self.terms
                .splice(0..0, core::iter::repeat(0).take(added_zeros));
        }
        let index = self.terms.len() - degree - 1;
        self.terms[index] += coeff;
    }

    // Evaluate the polynomial at a given value of x
    pub fn evaluate(&self, x: u64, p: u64) -> u64 {
        self.terms
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &coeff)| {
                let term_x = fmath::pow(x, i.try_into().unwrap(), p);
                fmath::mul(coeff, term_x, p)
            })
            .fold(0, |acc, x| fmath::add(acc, x, p))
    }

    pub fn trim(&mut self) {
        let inx = poly_fmath::first_nonzero_index(&self.terms);
        if inx != 0 {
            self.terms.drain(0..inx);
        }
    }

    pub fn is_zero(&self) -> bool {
        self.degree() == 0
    }

    pub fn get_term(&self, degree: usize) -> u64 {
        assert!(
            degree < self.terms.len(),
            "Degree is greater than the polynomial degree."
        );
        self.terms[self.terms.len() - degree - 1]
    }
}

impl std::fmt::Display for FPoly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let deg = self.degree();

        for (i, &term) in self.terms.iter().enumerate() {
            if term == 0 || i > deg {
                continue;
            }

            if !result.is_empty() {
                result.push_str(" + ");
            }

            match (term, deg - i) {
                (1, 0) => result.push_str("1"),
                (1, 1) => result.push_str("x"),
                (1, exp) => result.push_str(&format!("x^{}", exp)),
                (_, 0) => result.push_str(&term.to_string()),
                (_, 1) => result.push_str(&format!("{}x", term)),
                (_, exp) => result.push_str(&format!("{}x^{}", term, exp)),
            }
        }

        write!(f, "{}", result)
    }
}

#[macro_use]
pub mod poly_fmath {
    use super::{FPoly, Term};
    use crate::field::fmath;

    pub fn add(a: &FPoly, b: &FPoly, p: u64) -> FPoly {
        let (mut terms, small) = if b.terms.len() > a.terms.len() {
            (b.terms.clone(), &a.terms)
        } else {
            (a.terms.clone(), &b.terms)
        };

        let offset = terms.len() - small.len();

        for (index, &val) in terms[offset..].iter_mut().zip(small) {
            *index = fmath::add(*index, val, p);
        }

        FPoly::new(terms)
    }

    pub fn sub(a: &FPoly, b: &FPoly, p: u64) -> FPoly {
        let max_degree = std::cmp::max(a.terms.len(), b.terms.len());

        // Prepare the result vector
        let mut result_terms = vec![0; max_degree];

        // Align the terms from the right (highest degree)
        let a_offset = max_degree - a.terms.len();
        let b_offset = max_degree - b.terms.len();

        for i in 0..max_degree {
            let ai = if i >= a_offset {
                a.terms[i - a_offset]
            } else {
                0
            };
            let bi = if i >= b_offset {
                b.terms[i - b_offset]
            } else {
                0
            };

            // Perform subtraction modulo p
            result_terms[i] = fmath::sub(ai, bi, p);
        }

        FPoly {
            terms: result_terms,
        }
    }

    pub fn mul(a: &FPoly, b: &FPoly, p: u64) -> FPoly {
        let rhs = &a.terms[first_nonzero_index(&a.terms)..];
        let lhs = &b.terms[first_nonzero_index(&b.terms)..];
        let mut terms = vec![0; rhs.len() + lhs.len() - 1];
        for (index, &rterm) in rhs.iter().enumerate() {
            if rterm == 0 {
                continue;
            }
            for (&lterm, term) in lhs.iter().zip(terms[index..].iter_mut()) {
                let product = fmath::mul(rterm, lterm, p);
                *term = fmath::add(*term, product, p);
            }
        }
        FPoly::new(terms)
    }

    /// Performs the in-place Number Theoretic Transform (NTT) on a polynomial.
    ///
    /// The NTT is a modular variant of the Fast Fourier Transform (FFT) and is used to efficiently
    /// multiply polynomials in the modular arithmetic domain.
    ///
    /// # Parameters
    /// - `poly`: A mutable reference to a vector of polynomial coefficients.
    /// - `n`: The size of the transform, must be a power of two.
    /// - `root`: The primitive root modulo `mod_p` used for the transform.
    /// - `mod_p`: The modulus for the computation.
    ///
    /// # Details
    /// - Reorders the coefficients of `poly` using bit-reversal.
    /// - Applies the NTT using the provided root and modulus.
    fn ntt(poly: &mut Vec<u64>, n: usize, root: u64, p: u64) {
        let mut j = 0;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j ^= bit;
            if i < j {
                poly.swap(i, j);
            }
        }

        let mut length = 2;
        while length <= n {
            let w_len = fmath::pow(root, (p - 1) / length as u64, p);
            for i in (0..n).step_by(length) {
                let mut w = 1;
                for j in 0..length / 2 {
                    let u = poly[i + j];
                    let v = (poly[i + j + length / 2] * w) % p;
                    poly[i + j] = (u + v) % p;
                    poly[i + j + length / 2] = (u + p - v) % p;
                    w = (w * w_len) % p;
                }
            }
            length *= 2;
        }
    }

    /// Multiplies two polynomials using the Number Theoretic Transform (NTT) and modular arithmetic.
    ///
    /// # Parameters
    /// - `poly1`: The coefficients of the first polynomial.
    /// - `poly2`: The coefficients of the second polynomial.
    /// - `mod_p`: The modulus for the arithmetic.
    /// - `root`: The primitive root for the NTT.
    ///
    /// # Returns
    /// A vector containing the coefficients of the resulting polynomial.
    ///
    /// # Details
    /// - Expands both input polynomials to the next power of two.
    /// - Performs NTT on both polynomials.
    /// - Multiplies the transformed coefficients element-wise.
    /// - Applies the inverse NTT and rescales the coefficients by `1/n`.
    fn mul_ntt(a: FPoly, b: FPoly, p: u64, root: u64) -> FPoly {
        let len = a.degree() + b.degree() - 1;
        let n = len.next_power_of_two();

        let mut a = a.terms;
        let mut b = b.terms;
        a.resize(n, 0);
        b.resize(n, 0);

        ntt(&mut a, n, root, p);
        ntt(&mut b, n, root, p);

        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = (a[i] * b[i]) % p;
        }

        let inv_n = fmath::pow(n as u64, p - 2, p);
        ntt(&mut result, n, fmath::pow(root, p - 2, p), p);
        result.iter_mut().for_each(|x| *x = (*x * inv_n) % p);

        FPoly::new(result.into_iter().take(len).collect())
    }

    pub fn div(a: &FPoly, b: &FPoly, p: u64) -> (FPoly, FPoly) {
        let zero = 0;

        let (rhs_first, rhs_deg) = match first_term(&b.terms) {
            Term::ZeroTerm => panic!("Can't divide polynomial by 0."),
            Term::Term(coeff, deg) => (coeff, deg),
        };

        let (mut coeff, mut self_degree) = match first_term(&a.terms) {
            Term::ZeroTerm => {
                return (FPoly::zero(), a.clone());
            }
            Term::Term(coeff, degree) => {
                if degree < rhs_deg {
                    return (FPoly::zero(), a.clone());
                }
                (coeff, degree)
            }
        };

        let mut remainder = a.terms.clone();
        let mut quotient = vec![zero; self_degree - rhs_deg + 1];
        let offset = self_degree;

        while self_degree >= rhs_deg {
            let scale = coeff / rhs_first;
            vec_sub_w_scale(&mut remainder, self_degree, &b.terms, rhs_deg, scale, p);
            quotient[offset - self_degree] = scale;
            match first_term(&remainder) {
                Term::ZeroTerm => break,
                Term::Term(coeffx, degree) => {
                    coeff = coeffx;
                    self_degree = degree;
                }
            }
        }

        let mut quotient_poly = FPoly::new(quotient);
        let mut remainder_poly = FPoly::new(remainder);

        quotient_poly.trim();
        remainder_poly.trim();

        (quotient_poly, remainder_poly)
    }

    fn vec_sub_w_scale(a: &mut [u64], a_deg: usize, b: &[u64], b_deg: usize, b_scale: u64, p: u64) {
        let l = a.len() - a_deg - 1;
        for (lhs_t, rhs_t) in a[l..].iter_mut().zip(b[b.len() - b_deg - 1..].iter()) {
            *lhs_t = fmath::sub(*lhs_t, (*rhs_t) * b_scale, p);
        }
    }

    fn first_term(poly_vec: &[u64]) -> Term<u64> {
        for (deg, ch) in poly_vec.chunks_exact(4).enumerate() {
            for (inx, &val) in ch.iter().enumerate() {
                if !(val == 0) {
                    return Term::Term(val, poly_vec.len() - deg * 4 - inx - 1);
                }
            }
        }

        let mut inx = poly_vec.chunks_exact(4).len() * 4;
        for &val in poly_vec.chunks_exact(4).remainder().iter() {
            if !(val == 0) {
                return Term::Term(val, poly_vec.len() - inx - 1);
            }
            inx += 1;
        }

        Term::ZeroTerm
    }

    pub fn mul_by_number(a: &FPoly, y: u64, p: u64) -> FPoly {
        FPoly::new(a.terms.iter().map(|&x| fmath::mul(x, y, p)).collect())
    }

    pub fn first_nonzero_index(coeffs: &[u64]) -> usize {
        for (degree, chunk) in coeffs.chunks_exact(4).enumerate() {
            for (index, &val) in chunk.iter().enumerate() {
                if !(val == 0) {
                    return degree * 4 + index;
                }
            }
        }

        let mut len = coeffs.chunks_exact(4).len() * 4;
        for &value in coeffs.chunks_exact(4).remainder().iter() {
            if !(value == 0) {
                return len;
            }
            len += 1;
        }

        len
    }

    #[macro_export]
    macro_rules! poly_add_many {
        ($p:expr, $x:expr) => {
            $x
        };
        ($p:expr, $first:expr, $($rest:expr),+) => {
            crate::polynomial::poly_fmath::add(&$first, &poly_add_many!($p, $($rest),+), $p)
        };
    }

    #[macro_export]
    macro_rules! poly_mul_many {
        ($p:expr, $x:expr) => {
            $x
        };
        ($p:expr, $first:expr, $($rest:expr),+) => {
            crate::polynomial::poly_fmath::mul($first, &poly_mul_many!($p, $($rest),+), $p)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_fmath::*;

    #[test]
    fn test_add_many() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![5, 6, 8]);
        let poly3 = FPoly::new(vec![4, 22]);

        let result = poly_add_many!(11, poly1.clone(), poly2.clone(), poly3);
        assert_eq!(result.terms, vec![6, 1, 1]);



        let result = poly_add_many!(11, 
            poly_fmath::mul_by_number(&poly1, 11, 11), 
            poly2, 
            poly3
        );
        assert_eq!(result.terms, vec![5, 10, 30 % 11]);
    }

    #[test]
    fn test_mul_many() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![5, 6, 8]);
        let poly3 = FPoly::new(vec![4, 22]);

        let result = poly_mul_many!(11, &poly1, &poly2, &poly3);
        assert_eq!(result.terms, vec![9, 9, 6, 6, 7, 0]);
    }

    #[test]
    fn test_eval() {
        let poly1 = FPoly::new(vec![10, 70, 12, 220, 133, 112, 512, 150]);

        assert_eq!(poly1.evaluate(2, 181), 42);
        assert_eq!(poly1.evaluate(191, 181), 154);
        assert_eq!(poly1.evaluate(0, 181), 150);
        assert_eq!(poly1.evaluate(0, 11), 7);
    }

    #[test]
    fn test_degree() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![0]);
        let poly3 = FPoly::new(vec![]);

        assert_eq!(poly1.degree(), 2);
        assert_eq!(poly2.degree(), 0);
        assert_eq!(poly3.degree(), 0);
    }

    #[test]
    fn test_add() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![5, 6, 8]);
        let poly3 = FPoly::new(vec![4, 22]);

        assert_eq!(vec![6, 8, 1], add(&poly1, &poly2, 11).terms);
        assert_eq!(vec![5, 10, 8], add(&poly2, &poly3, 11).terms);

        let poly1 = FPoly::new(vec![0]);
        let poly2 = FPoly::new(vec![0, 0]);

        let mut result = poly_fmath::add(&poly1, &poly2, 11);
        result.trim();

        assert!(result.terms.len() == 0);
    }

    #[test]
    fn test_sub() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![5, 6, 8]);
        let poly3 = FPoly::new(vec![4, 22]);

        assert_eq!(vec![7, 7, 7], sub(&poly1, &poly2, 11).terms);
        assert_eq!(vec![5, 2, 8], sub(&poly2, &poly3, 11).terms);
        assert_eq!(vec![6, 9, 3], sub(&poly3, &poly2, 11).terms);
    }

    #[test]
    fn test_mul() {
        let poly1 = FPoly::new(vec![1, 5, 6, 9]);
        let poly2 = FPoly::new(vec![2, 7, 11, 5, 24]);
        let poly3 = FPoly::new(vec![]);

        assert_eq!(vec![2, 6, 3, 10, 2, 7, 2, 7], mul(&poly1, &poly2, 11).terms);
        assert_eq!(vec![0, 0, 0], mul(&poly1, &poly3, 11).terms);
    }

    #[test]
    fn test_div() {
        let poly1 = FPoly::new(vec![1, 5, 6, 9]);
        let poly2 = FPoly::new(vec![2, 7, 11, 5, 24]);

        assert_eq!(0, div(&poly1, &poly2, 11).0.degree());

        assert_eq!(vec![1, 5, 6, 9], div(&poly1, &poly2, 11).1.terms);

        assert_eq!(vec![2, 8], div(&poly2, &poly1, 11).0.terms);

        assert_eq!(vec![3, 5, 7], div(&poly2, &poly1, 11).1.terms);
    }
}
