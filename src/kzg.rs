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
        res_poly = (0..=deg)
            .filter_map(|i| {
                if let Term::Term(t, _) = poly_in.term_with_degree(i) {
                    Some(Mfp::from(to_bint!(t) * to_bint!(*ck.get(i)?)))
                } else {
                    None
                }
            })
            .sum();
    }

    res_poly
}
