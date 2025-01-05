use crate::field::fmath;


#[derive(Debug, Clone, PartialEq)]
/// Term is a type which represents a term in a polynomial.
pub enum Term<N> {
    /// A term with coefficient zero. Has degree -inf.
    ZeroTerm,
    /// A term with non-zero coefficient and a degree.
    Term(N, usize),
}

#[derive(Debug, Clone)]
pub struct FPoly {
    terms: Vec<u64>,
}

impl FPoly {
    pub fn new(terms: Vec<u64>) -> Self {
        Self { terms }
    }

    pub fn zero() -> Self {
        Self { terms: vec![] }
    }

    pub fn one() -> Self {
        Self { terms: vec![1] }
    }

    pub fn one_x() -> Self {
        Self { terms: vec![1, 0] }
    }

    pub fn degree(&self) -> usize {
        self.terms.len()
    }

    // Evaluate the polynomial at a given value of x
    pub fn evaluate(&self, x: u64, p: u64) -> u64 {
        // FIXME
        self.terms
            .iter()
            .enumerate()
            .map(|(i, &coeff)| coeff * fmath::pow(x, i.try_into().unwrap(), p))
            .sum()
    }

    pub fn trim(&mut self) {
        let inx = polynomial_math::first_nonzero_index(&self.terms);
        if inx != 0 {
            self.terms.drain(0..inx);
        }
    }
}

mod polynomial_math {
    use super::{FPoly, Term};
    use crate::field::fmath;

    pub fn poly_add(a: &FPoly, b: &FPoly, p: u64) -> FPoly {
        let (terms, small) = if b.degree() > a.degree() {
            (&b.terms, &a.terms)
        } else {
            (&a.terms, &b.terms)
        };

        let offset = terms.len() - small.len();

        let mut terms = terms.clone();

        for (index, &val) in terms[offset..].iter_mut().zip(small) {
            *index = fmath::add(*index, val, p)
        }

        FPoly::new(terms)
    }

    pub fn poly_sub(a: &FPoly, b: &FPoly, p: u64) -> FPoly {
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

    pub fn poly_mul(a: &FPoly, b: &FPoly, p: u64) -> FPoly {
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

    pub fn poly_mul_ntt(a: &FPoly, b: &FPoly, p: u64, root: u64) -> FPoly {
        unimplemented!()
    }


    pub fn poly_div(a: &FPoly, b: &FPoly, p: u64) -> (FPoly, FPoly) {
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

    fn vec_sub_w_scale(
        a: &mut [u64],
        a_deg: usize,
        b: &[u64],
        b_deg: usize,
        b_scale: u64,
        p: u64
    ) {
        let l = a.len() - a_deg - 1;
        for (lhs_t, rhs_t) in a[l..]
            .iter_mut()
            .zip(b[b.len() - b_deg - 1..].iter())
        {
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

    pub fn poly_mul_by_number(a: &FPoly, b: u64, p: u64) -> FPoly {
        unimplemented!()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use polynomial_math::*;

    #[test]
    fn test_add() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![5, 6, 8]);
        let poly3 = FPoly::new(vec![4, 22]);

        assert_eq!(vec![6, 8, 1], poly_add(&poly1, &poly2, 11).terms);
        assert_eq!(vec![5, 10, 8], poly_add(&poly2, &poly3, 11).terms);
    }

    #[test]
    fn test_sub() {
        let poly1 = FPoly::new(vec![1, 2, 4]);
        let poly2 = FPoly::new(vec![5, 6, 8]);
        let poly3 = FPoly::new(vec![4, 22]);

        assert_eq!(vec![7, 7, 7], poly_sub(&poly1, &poly2, 11).terms);
        assert_eq!(vec![5, 2, 8], poly_sub(&poly2, &poly3, 11).terms);
        assert_eq!(vec![6, 9, 3], poly_sub(&poly3, &poly2, 11).terms);
    }

    #[test]
    fn test_mul() {
        let poly1 = FPoly::new(vec![1, 5, 6, 9]);
        let poly2 = FPoly::new(vec![2, 7, 11, 5, 24]);
        let poly3 = FPoly::new(vec![]);

        assert_eq!(
            vec![2, 6, 3, 10, 2, 7, 2, 7],
            poly_mul(&poly1, &poly2, 11).terms
        );
        assert_eq!(vec![0, 0, 0], poly_mul(&poly1, &poly3, 11).terms);
    }

    #[test]
    fn test_div() {
        let poly1 = FPoly::new(vec![1, 5, 6, 9]);
        let poly2 = FPoly::new(vec![2, 7, 11, 5, 24]);
        
        assert_eq!(
            0,
            poly_div(&poly1, &poly2, 11).0.terms.len()
        );

        assert_eq!(
            vec![1, 5, 6, 9],
            poly_div(&poly1, &poly2, 11).1.terms
        );



        assert_eq!(
            vec![2, 8],
            poly_div(&poly2, &poly1, 11).0.terms
        );

        assert_eq!(
            vec![3, 5, 7],
            poly_div(&poly2, &poly1, 11).1.terms
        );
    }
}