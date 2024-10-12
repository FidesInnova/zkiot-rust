//! Module for mathematical functions and utilities for finite field operations using the `Mfp` type and polynomials.

use ark_ff::{Field, PrimeField};
use nalgebra::{Complex, DMatrix};
use rand::{seq::SliceRandom, thread_rng};
use rustnomial::*;
use std::collections::HashMap;
use std::ops::Neg;
use ark_ff::Zero;

use rayon::prelude::*; // For parallel iteration

use crate::{dsp_poly, field, to_bint, utils::add_random_points};

/// Define the constant modulus for field operations.

// 2013265921
// pub const P: u64 = 18446744069414584321;
// pub const P: u64 = 2013265921;

// pub const P: u64 = 45151681;
// pub const GENERATOR: u64 = 61;

pub const P: u64 = 881;
pub const GENERATOR: u64 = 3;

// pub const P: u64 = 181;
// pub const GENERATOR: u64 = 2;

field!(Mfp, P);

/// Type alias for a polynomial over the `Mfp` field.
pub type Poly = Polynomial<Mfp>;

/// Type alias for a 2D point in the `Mfp` field.
pub type Point = (Mfp, Mfp);


/// Computes the modular exponentiation of `a` raised to the power `b`
/// and returns the result as an element of the finite field `Mfp`.
///
/// # Parameters
/// - `a`: The base value, of type `u64`.
/// - `b`: The exponent value, of type `u64`.
///
/// # Returns
/// Returns the result of `a^b` as an element of the finite field `Mfp`.
///
/// # Example
/// ```
/// use zk_iot::math::Mfp;
/// use zk_iot::math::exp_mod;
/// use zk_iot::*;
///
/// let result = exp_mod(2, 10);
/// assert_eq!(result, Mfp::from(1024));
/// ```
pub fn exp_mod(a: u64, b: u64) -> Mfp {
    Mfp::from(a).pow([b])
}

/// Constructs a polynomial of the form u(x , y) = (x^degree - y^degree) / (x - y),
/// where either `x` or `y` is provided as an option.
///
/// # Parameters
/// - `x`: An optional value in the finite field `Mfp`. Only one of `x` or `y` should be `Some`.
/// - `y`: An optional value in the finite field `Mfp`. Only one of `x` or `y` should be `Some`.
/// - `degree`: The degree of the polynomial, of type `usize`.
///
/// # Returns
/// Returns the result of the polynomial division as a `Poly` object.
///
/// # Panics
/// The function panics if both `x` and `y` are either `None` or `Some`.
/// This ensures that exactly one of the parameters is provided.
pub fn func_u(x: Option<Mfp>, y: Option<Mfp>, degree: usize) -> Poly {
    if x.is_none() && y.is_none() {
        panic!("At least one of x or y must be Some.");
    }

    let mut numerator = Poly::new(vec![]);
    let mut denominator = Poly::new(vec![]);

    if let Some(x) = x {
        numerator.add_term(exp_mod(to_bint!(x), degree as u64), 0);
        numerator.add_term(-Mfp::ONE, degree);

        denominator.add_term(x, 0);
        denominator.add_term(-Mfp::ONE, 1);
    }
    if let Some(y) = y {
        numerator.add_term(-exp_mod(to_bint!(y), degree as u64), 0);
        numerator.add_term(Mfp::ONE, degree);

        denominator.add_term(-y, 0);
        denominator.add_term(Mfp::ONE, 1);
    }
    if numerator.degree() == denominator.degree() && denominator.degree() == Degree::Num(0) {
        let div_res = div_mod_val(numerator.terms_as_vec().get(0).unwrap().0, denominator.terms_as_vec().get(0).unwrap().0);
        return Poly::from(vec![div_res]);
    }
    div_mod(&numerator, &denominator).0
}

pub fn div_mod(a: &Poly, rhs: &Poly) -> (Poly, Poly) {
    let zero = Mfp::ZERO;

    let (rhs_first, rhs_deg) = match first_term(&rhs.terms) {
        Term::ZeroTerm => panic!("Can't divide polynomial by 0."),
        Term::Term(coeff, deg) => (coeff, deg),
    };

    let (mut coeff, mut self_degree) = match first_term(&a.terms) {
        Term::ZeroTerm => {
            return (Polynomial::zero(), a.clone());
        }
        Term::Term(coeff, degree) => {
            if degree < rhs_deg {
                return (Polynomial::zero(), a.clone());
            }
            (coeff, degree)
        }
    };

    let mut remainder = a.terms.clone();
    let mut div = vec![zero; self_degree - rhs_deg + 1];
    let offset = self_degree;

    while self_degree >= rhs_deg {
        let div_res = div_mod_val(coeff, rhs_first);
        let scale = div_res;
        vec_sub_w_scale(&mut remainder, self_degree, &rhs.terms, rhs_deg, scale);
        div[offset - self_degree] = scale;
        match first_term(&remainder) {
            Term::ZeroTerm => break,
            Term::Term(coeffx, degree) => {
                coeff = coeffx;
                self_degree = degree;
            }
        }
    }

    (Poly::new(div), Poly::new(remainder))
}

fn vec_sub_w_scale(
    lhs: &mut [Mfp],
    lhs_degree: usize,
    rhs: &[Mfp],
    rhs_deg: usize,
    rhs_scale: Mfp,
) {
    let loc = lhs.len() - lhs_degree - 1;
    for (lhs_t, rhs_t) in lhs[loc..]
        .iter_mut()
        .zip(rhs[rhs.len() - rhs_deg - 1..].iter())
    {
        *lhs_t -= (*rhs_t) * rhs_scale;
    }
}

fn first_term(poly_vec: &[Mfp]) -> Term<Mfp> {
    for (degree, chunk) in poly_vec.chunks_exact(4).enumerate() {
        for (index, &value) in chunk.iter().enumerate() {
            if !value.is_zero() {
                return Term::Term(value, poly_vec.len() - degree * 4 - index - 1);
            }
        }
    }

    let mut index = poly_vec.chunks_exact(4).len() * 4;
    for &value in poly_vec.chunks_exact(4).remainder().iter() {
        if !value.is_zero() {
            return Term::Term(value, poly_vec.len() - index - 1);
        }
        index += 1;
    }

    Term::ZeroTerm
}


pub fn newton_interpolate(points: &[Point]) -> Poly {
    let n = points.len();
    let mut divided_differences = vec![vec![Mfp::ZERO; n]; n];

    // Initialize the divided differences table with y-values
    for (i, (_, y)) in points.iter().enumerate() {
        divided_differences[i][0] = *y;
    }

    // Compute the divided differences table
    for j in 1..n {
        for i in 0..(n - j) {
            let x_i = Mfp::from(points[i].0);
            let x_ij = Mfp::from(points[i + j].0);
            let numerator = divided_differences[i + 1][j - 1] - divided_differences[i][j - 1];
            let denominator = x_ij - x_i;
            divided_differences[i][j] = div_mod_val(numerator, denominator);
        }
    }

    // Build the Newton polynomial
    let mut poly_res = Poly::new(vec![divided_differences[0][0]]);
    let mut poly_term = Poly::new(vec![Mfp::ONE]);

    for i in 1..n {
        let x_i = Mfp::from(points[i - 1].0);
        let new_term = Poly::new(vec![Mfp::ONE, x_i.neg()]);
        poly_term *= new_term; // Multiply by (x - x_i) for each term
        poly_res += poly_term.clone() * divided_differences[0][i];
    }

    poly_res
}

/// Generates a vector of elements in the finite field `Mfp` based on the given
/// generator and length.
///
/// # Parameters
/// - `ms_gen`: The generator value for the finite field.
/// - `len`: The number of elements to generate.
///
/// # Returns
/// Returns a vector of `Mfp` elements, where each element is generated by raising
/// the generator `ms_gen` to increasing powers from 0 to `len - 1`.
///
/// # Description
/// This function generates a set of field elements using the specified generator
/// and length. Each element in the resulting vector is computed as `ms_gen^i`, where
/// `i` ranges from 0 to `len - 1`.
pub fn generate_set(len: u64) -> Vec<Mfp> {
    let g = to_bint!(exp_mod(GENERATOR, (P - 1) / len)); // Compute the generator for set H
    (0..len).map(|i| exp_mod(g, i)).collect()
}


/// Computes the vanishing polynomial for a given set of field elements.
///
/// # Parameters
/// - `set`: A reference to a vector of `Mfp` elements representing the roots of the polynomial.
///
/// # Returns
/// Returns a `Poly` representing the vanishing polynomial with the provided roots.
///
/// # Description
/// This function constructs a polynomial with the given field elements as its roots. The resulting polynomial
/// will be zero at each of these field elements. The polynomial is built by multiplying linear factors corresponding
/// to each root.
pub fn vanishing_poly(set: &Vec<Mfp>) -> Poly {
    let mut vp = Poly::new(vec![Mfp::ONE]);

    for i in set {
        vp *= Poly::new(vec![Mfp::ONE, Mfp::ZERO]) - Poly::new(vec![*i]);
    }
    vp.trim();
    vp
}

/// Computes the value at specific points of a matrix `mat` based on the sets `set_h` and `set_k`,
/// and the mappings `row_k` and `col_k`. It evaluates a polynomial `poly_u` at these points
/// and divides the matrix value by the product of the evaluated values.
///
/// # Parameters
/// - `mat`: A reference to the matrix `mat` of type `DMatrix<Mfp>`.
/// - `set_h`: A vector of values in the finite field `Mfp`, used as part of the polynomial evaluation.
/// - `set_k`: A vector of values in the finite field `Mfp`, used to identify the specific point in the matrix.
/// - `row_k`: A reference to a `HashMap` that maps values in `set_k` to the corresponding row values.
/// - `col_k`: A reference to a `HashMap` that maps values in `set_k` to the corresponding column values.
///
/// # Returns
/// Returns a `HashMap<Mfp, Mfp>` where each key is a value from `set_k` and the value is the
/// computed polynomial division result at that matrix point.
///
/// # Description
/// The function iterates over the matrix `mat` and, for each non-zero element, computes a value
/// based on the evaluation of the polynomial `poly_u` at the points defined by `row_k` and `col_k`.
/// It then stores the result in the `res` map, associating it with the corresponding value from `set_k`.
///
/// # Panic
/// The function will panic if an index `c` in `set_k` is out of bounds or if `set_k` does not have
/// a value corresponding to the index `c`. This is asserted with `assert!(set_k.get(c).is_some());`.
pub fn get_matrix_point_val(
    mat: &DMatrix<Mfp>,
    set_h: &[Mfp],
    set_k: &[Mfp],
    row_k: &HashMap<Mfp, Mfp>,
    col_k: &HashMap<Mfp, Mfp>,
) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    let len = set_h.len();
    let mut poly_u = Poly::from(vec![Mfp::ZERO]);
    poly_u.add_term(Mfp::from(len as u64), len - 1);

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                let val = mat[(i, j)];
                assert!(set_k.get(c).is_some());
                let k = set_k[c];
                let div_res = div_mod_val(val, poly_u.eval(row_k[&k]) * poly_u.eval(col_k[&k]));
                let p2 = div_res;
                res.insert(set_k[c], p2);
                c += 1;
            }
        }
    }

    res
}

/// Maps non-zero elements of the matrix `mat` to the corresponding row values from `set_h`
/// based on the index in `set_k`.
///
/// # Parameters
/// - `mat`: A reference to the matrix `mat` of type `DMatrix<Mfp>`.
/// - `set_h`: A vector of values representing the rows in the finite field `Mfp`.
/// - `set_k`: A vector of values used to identify specific points in the matrix.
///
/// # Returns
/// Returns a `HashMap<Mfp, Mfp>` where each key is a value from `set_k` and the corresponding
/// value is the row value from `set_h`.
///
/// # Description
/// The function iterates over the matrix `mat` and, for each non-zero element,
/// maps the corresponding value in `set_k` to the row value in `set_h`.
pub fn get_matrix_point_row(mat: &DMatrix<Mfp>, set_h: &[Mfp], set_k: &[Mfp]) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert(set_k[c], set_h[i]);
                c += 1;
            }
        }
    }

    add_random_points(&mut res, c, set_h, set_k).unwrap();

    res
}

/// Maps non-zero elements of the matrix `mat` to the corresponding column values from `set_h`
/// based on the index in `set_k`.
///
/// # Parameters
/// - `mat`: A reference to the matrix `mat` of type `DMatrix<Mfp>`.
/// - `set_h`: A vector of values representing the columns in the finite field `Mfp`.
/// - `set_k`: A vector of values used to identify specific points in the matrix.
///
/// # Returns
/// Returns a `HashMap<Mfp, Mfp>` where each key is a value from `set_k` and the corresponding
/// value is the column value from `set_h`.
///
/// # Description
/// The function iterates over the matrix `mat` and, for each non-zero element,
/// maps the corresponding value in `set_k` to the column value in `set_h`.
pub fn get_matrix_point_col(mat: &DMatrix<Mfp>, set_h: &[Mfp], set_k: &[Mfp]) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert(set_k[c], set_h[j]);
                c += 1;
            }
        }
    }

    add_random_points(&mut res, c, set_h, set_k).unwrap();

    res
}

/// Computes a polynomial `m_xk` based on the provided `points_val`, `points_row`, and `points_col`.
///
/// # Parameters
/// - `num`: A reference to an `Mfp` element, used to evaluate the resulting polynomial.
/// - `points_val`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `points_row`: A `HashMap` mapping points to their corresponding row values in the matrix.
/// - `points_col`: A `HashMap` mapping points to their corresponding column values in the matrix.
/// - `set_h_len`: The length of the set `H`, which determines the degree of the polynomial.
///
/// # Returns
/// Returns a `Poly` representing the result of summing up the products of the evaluated polynomials.
///
/// # Description
/// This function iterates over each key-value pair `(k, val)` in `points_val`, and for each pair:
/// 1. Constructs a polynomial `poly_val` from the value `val`.
/// 2. Constructs two polynomials `poly_x` and `poly_y` using the `func_u` function, with `points_row[k]` and `points_col[k]` as inputs, respectively.
/// 3. Evaluates `poly_y` at `num` to get `res_poly_y`, then multiplies it by `poly_val` and `poly_x`.
/// 4. Sums up these products to obtain the final polynomial `poly_res`.
///
/// The function `m_kx` follows a similar process, but evaluates `poly_x` at `num` instead of `poly_y`.
///
/// # Notes
/// - `m_xk`: The final polynomial depends on `poly_x` and the evaluation of `poly_y` at `num`.
/// - `m_kx`: The final polynomial depends on `poly_y` and the evaluation of `poly_x` at `num`.
///
/// These two functions are used to compute different aspects of the overall polynomial interaction.
pub fn m_xk(
    num: &Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
    set_h_len: usize,
) -> Poly {
    let mut poly_res = Poly::from(vec![Mfp::ZERO]);

    for (k, val) in points_val {
        let poly_val = Poly::from(vec![*val]);
        let poly_x = func_u(None, Some(points_row[k]), set_h_len);
        let poly_y = func_u(None, Some(points_col[k]), set_h_len);
        let res_poly_y = poly_y.eval(*num);
        poly_res += poly_val * res_poly_y * poly_x;
    }

    poly_res
}

/// Computes a polynomial `m_kx` based on the provided `points_val`, `points_row`, and `points_col`.
///
/// # Parameters
/// - `num`: A reference to an `Mfp` element, used to evaluate the resulting polynomial.
/// - `points_val`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `points_row`: A `HashMap` mapping points to their corresponding row values in the matrix.
/// - `points_col`: A `HashMap` mapping points to their corresponding column values in the matrix.
/// - `set_h_len`: The length of the set `H`, which determines the degree of the polynomial.
///
/// # Returns
/// Returns a `Poly` representing the result of summing up the products of the evaluated polynomials.
///
/// # Description
/// This function iterates over each key-value pair `(k, val)` in `points_val`, and for each pair:
/// 1. Constructs a polynomial `poly_val` from the value `val`.
/// 2. Constructs two polynomials `poly_y` and `poly_y` using the `func_u` function, with `points_row[k]` and `points_col[k]` as inputs, respectively.
/// 3. Evaluates `poly_y` at `num` to get `res_poly_y`, then multiplies it by `poly_val` and `poly_y`.
/// 4. Sums up these products to obtain the final polynomial `poly_res`.
///
/// The function `m_kx` follows a similar process, but evaluates `poly_y` at `num` instead of `poly_y`.
///
/// # Notes
/// - `m_xk`: The final polynomial depends on `poly_y` and the evaluation of `poly_y` at `num`.
/// - `m_kx`: The final polynomial depends on `poly_y` and the evaluation of `poly_y` at `num`.
///
/// These two functions are used to compute different aspects of the overall polynomial interaction.
pub fn m_kx(
    num: &Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
    set_h_len: usize,
) -> Poly {
    let mut poly_res = Poly::from(vec![Mfp::ZERO]);

    for (k, val) in points_val {
        let poly_val = Poly::from(vec![*val]);
        let poly_x = func_u(None, Some(points_row[k]), set_h_len);
        let poly_y = func_u(None, Some(points_col[k]), set_h_len);
        let res_poly_x = poly_x.eval(*num);
        poly_res += poly_val * res_poly_x * poly_y;
    }

    poly_res
}

/// Computes the polynomial sum for `sigma_rkx_mkx` based on the provided set `H`, `alpha`, and points.
///
/// # Parameters
/// - `set_h`: A reference to a vector of `Mfp` elements representing the set `H`.
/// - `alpha`: An `Mfp` element used in the polynomial computation.
/// - `points_val`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `points_row`: A `HashMap` mapping points to their corresponding row values in the matrix.
/// - `points_col`: A `HashMap` mapping points to their corresponding column values in the matrix.
///
/// # Returns
/// Returns a `Poly` representing the sum of the products of polynomials.
///
/// # Description
/// This function iterates over each element `h` in `set_h`, and for each `h`:
/// 1. Constructs a polynomial `p_r_alphak` using `func_u`, which depends on `alpha` and `h`.
/// 2. Constructs a polynomial `p_m_kx` using the `m_kx` function.
/// 3. Trims the polynomials to remove leading zeros.
/// 4. Multiplies `p_r_alphak` and `p_m_kx` and sums the result into `res`.
///
/// This function is used to compute the final polynomial based on the interaction between `alpha` and `h`.
pub fn sigma_rkx_mkx(
    set_h: &Vec<Mfp>,
    alpha: Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
) -> Poly {
    let mut res = Poly::from(vec![Mfp::ZERO]);
    for h in set_h {
        let mut p_r_alphak = func_u(Some(alpha), Some(*h), set_h.len());
        let mut p_m_kx = m_kx(h, points_val, points_row, points_col, set_h.len());
        p_r_alphak.trim();
        p_m_kx.trim();
        let mul_poly = p_r_alphak * p_m_kx;
        res += mul_poly;
    }
    res
}

/// Computes the polynomial sum for `sigma_rxk_mxk` based on the provided set `H`, `alpha`, and points.
///
/// # Parameters
/// - `set_h`: A reference to a vector of `Mfp` elements representing the set `H`.
/// - `alpha`: An `Mfp` element used in the polynomial computation.
/// - `points_val`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `points_row`: A `HashMap` mapping points to their corresponding row values in the matrix.
/// - `points_col`: A `HashMap` mapping points to their corresponding column values in the matrix.
///
/// # Returns
/// Returns a `Poly` representing the sum of the products of polynomials.
///
/// # Description
/// This function iterates over each element `h` in `set_h`, and for each `h`:
/// 1. Constructs a polynomial `p_r_alphak` using `func_u`, which depends on `alpha` and `h`.
/// 2. Constructs a polynomial `p_m_xk` using the `m_xk` function.
/// 3. Trims the polynomials to remove leading zeros.
/// 4. Multiplies `p_r_alphak` and `p_m_xk` and sums the result into `res`.
///
/// This function is used to compute the final polynomial based on the interaction between `alpha` and `h`.
pub fn sigma_rxk_mxk(
    set_h: &Vec<Mfp>,
    alpha: Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
) -> Poly {
    let mut res = Poly::from(vec![Mfp::ZERO]);
    for h in set_h {
        let mut p_r_alphak = func_u(Some(alpha), Some(*h), set_h.len());
        let mut p_m_xk = m_xk(h, points_val, points_row, points_col, set_h.len());
        p_r_alphak.trim();
        p_m_xk.trim();
        res += p_r_alphak * p_m_xk;
    }
    res
}

/// Calculates the sigma_m value based on the provided polynomials and parameters.
///
/// # Parameters
/// - `van_poly_vhx`: The Vandermonde polynomial evaluated at `v_h(x)`.
/// - `eta`: An `Mfp` value used as a multiplier in the result.
/// - `beta_1`: The first `Mfp` value to evaluate the Vandermonde polynomial.
/// - `beta_2`: The second `Mfp` value to evaluate the Vandermonde polynomial.
/// - `k`: The `Mfp` value used to evaluate the polynomials in the `polys` array.
/// - `polys`: A slice containing references to the row, column, and value polynomials
///   (in that order).
///
/// # Returns
/// Returns an `Mfp` value calculated as `eta * (nu / de)`, where `nu` is the product
/// of the Vandermonde polynomials evaluated at `beta_1` and `beta_2`, and `polys[2]` evaluated at `k`.
/// `de` is the product of differences between `beta_2` and `polys[0]` evaluated at `k`,
/// and `beta_1` and `polys[1]` evaluated at `k`.
pub fn sigma_m(
    van_poly_vhx: &Poly,
    eta: &Mfp,
    beta_1: &Mfp,
    beta_2: &Mfp,
    k: &Mfp,
    polys: &[&Poly],
) -> Mfp {
    let nu = van_poly_vhx.eval(*beta_1) * van_poly_vhx.eval(*beta_2) * polys[2].eval(*k);
    let de = (beta_2 - &polys[0].eval(*k)) * (beta_1 - &polys[1].eval(*k));
    let de = exp_mod(to_bint!(de), P - 2);
    let div = nu * de;
    eta * &div
}

/// Computes the Lagrange interpolation polynomial `L_i(y_i)` for a given set of points.
///
/// # Parameters
/// - `points`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `set_k`: A reference to a vector of `Mfp` elements used to identify the points in the Lagrange interpolation.
///
/// # Returns
/// Returns a `Poly` object representing the Lagrange interpolation polynomial
/// based on the points in `set_k` and their corresponding values in `points`.
///
/// # Description
/// This function constructs a Lagrange interpolation polynomial using the points provided
/// in `set_k` and the corresponding values found in the `points` HashMap. If a point in `set_k`
/// does not have a corresponding value in `points`, it defaults to `Mfp::ZERO`.
pub fn sigma_yi_li(points: &HashMap<Mfp, Mfp>, set_k: &Vec<Mfp>) -> Poly {
    let mut points_li: Vec<Point> = vec![];
    for k in set_k {
        let val = points.get(k).unwrap_or(&Mfp::ZERO);
        points_li.push((*k, *val));
    }
    newton_interpolate(&points_li)
}


pub fn e_func(a: Mfp, b: Mfp, g: Mfp) -> Mfp {
    println!("a: {a}, b: {b}");
    let a_r = Mfp::from(div_mod_val(a, g));
    let b_r = Mfp::from(div_mod_val(b, g));
    println!("a_r: {a_r}, b_r: {b_r}");
    let exp = a_r * b_r;
    Mfp::from(3) * exp 
}

pub fn invers_val(a: Mfp) -> Mfp {
    exp_mod(to_bint!(a), P - 2)
}

pub fn div_mod_val(a: Mfp, b: Mfp) -> Mfp {
    let numerator = a;
    let denominator = invers_val(b);
    numerator * denominator
}

pub fn compute_all_commitment(polys: &[Poly], ck: &Vec<Mfp>, g: u64) -> Vec<Mfp> {
    let mut res = vec![];

    for poly in polys.iter() {
        let commitment_num = kzg::commit(&poly, &ck);
        res.push(commitment_num);
    }

    res
}

pub mod kzg {
    use super::*;
    pub fn setup(max: u64, tau: u64, g: u64) -> Vec<Mfp> {
        let mut res = vec![];
        let mut tmp = Mfp::from(g);
        let tau = tau % (P - 1);

        for _ in 0..max {
            res.push(tmp);
            tmp = Mfp::from(to_bint!(tmp) * tau);
        }

        res
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
}

/// Computes the logarithm of `b` in base `a` using a modified approach based on the
/// Baby-step Giant-step algorithm.
///
/// # Parameters
/// - `a`: The base of the logarithm, represented as an `Mfp` type.
/// - `b`: The value for which the logarithm is to be computed, also represented as an `Mfp` type.
///
/// # Returns
/// An `Mfp` representing the logarithm of `b` in base `a`, or `Mfp::ZERO` if the
/// logarithm cannot be determined.
pub fn log_mod(a: Mfp, b: Mfp) -> Mfp {
    let m = ((P - 1) as f64).sqrt().ceil() as u64;

    let mut tbl = std::collections::HashMap::new();

    for i in 0..m {
        tbl.insert(exp_mod(to_bint!(a), i), i);
    }
    let c = exp_mod(to_bint!(a), m * (P - 2));

    for j in 0..m {
        let y = b * exp_mod(to_bint!(c), j);

        if tbl.contains_key(&y) {
            let num = *tbl.get(&y).unwrap();
            return Mfp::from(j * m + num);
        }
    }

    Mfp::ZERO
}

#[cfg(test)]
mod math_test {
    use super::*;
    use ark_ff::UniformRand;
    use rand::Rng;

    #[test]
    fn test_div_val() {
        // Test cases
        let test_cases = vec![
            (Mfp::from(10), Mfp::from(2), Some(Mfp::from(5))), 
            (Mfp::from(7), Mfp::from(1), Some(Mfp::from(7))), 
            (Mfp::from(0), Mfp::from(5), Some(Mfp::from(0))), 
            (Mfp::from(5), Mfp::from(0), Some(Mfp::from(0))), 
            (Mfp::from(-10), Mfp::from(-2), Some(Mfp::from(5))),
            (Mfp::from(1_000_000_000), Mfp::from(1_000_000_000), Some(Mfp::from(1))), 
        ];

        for (a, b, expected) in test_cases {
            if let Some(expected_value) = expected {
                assert_eq!(div_mod_val(a, b), expected_value)
            }
        }
    }

    #[test]
    fn test_log_mod() {
        for _ in 0..100 {
            let a = Mfp::from(thread_rng().gen_range(1..P));
            let b = Mfp::from(thread_rng().gen_range(1..P));

            if log_mod(a, b) == Mfp::ZERO {
                assert_eq!(exp_mod(to_bint!(a), to_bint!(log_mod(a, b))), Mfp::ONE);
            } else {
                assert_eq!(exp_mod(to_bint!(a), to_bint!(log_mod(a, b))), b);
            }
        }
    }

    #[test]
    fn test_exp_mod() {
        assert_eq!(exp_mod(2, 0), Mfp::ONE);
        assert_eq!(exp_mod(10, 0), Mfp::ONE);

        assert_eq!(exp_mod(2, 1), Mfp::from(2));
        assert_eq!(exp_mod(10, 1), Mfp::from(10));

        assert_eq!(exp_mod(2, 3), Mfp::from(8));
        assert_eq!(exp_mod(3, 2), Mfp::from(9));
        assert_eq!(exp_mod(5, 3), Mfp::from(125));

        assert_eq!(exp_mod(2, 10), Mfp::from(1024));
        assert_eq!(exp_mod(3, 5), Mfp::from(243));

        assert_eq!(
            exp_mod(987654321987654321, 1234567890123456789),
            Mfp::from(42)
        );
        assert_eq!(
            exp_mod(887654448019654120, 1139562969472691009),
            Mfp::from(55)
        );
        assert_eq!(exp_mod(u64::MAX, 9223372036854775807), Mfp::from(65));
    }

    #[test]
    fn test_func_u() {
        assert_eq!(
            Poly::new(vec![Mfp::from(1)]),
            func_u(Some(Mfp::from(1)), Some(Mfp::from(0)), 100)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(1)]),
            func_u(Some(Mfp::from(0)), Some(Mfp::from(1)), 100)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(70)]),
            func_u(Some(Mfp::from(10)), Some(Mfp::from(1)), 5)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(53)]),
            func_u(Some(Mfp::from(123)), Some(Mfp::from(321)), 10)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(99)]),
            func_u(Some(Mfp::from(2838193)), Some(Mfp::from(9728224)), 50)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(63)]),
            func_u(Some(Mfp::from(!1)), Some(Mfp::from(!0)), 10)
        );
    }

    #[test]
    fn test_interpolate() {
        let points1 = [
            (1, 3),
            (4, 5),
            (10, 22),
            (111, 222),
            (0, 0),
            (0, 0),
            (1234, 4567),
            (122222, 1344556),
        ];
        let points2 = [
            (39942, 123244),
            (41221133, 53534213),
            (12121210, 2424222),
            (1242411, 242422),
            (0, 0),
        ];

        let points1: Vec<Point> = points1
            .iter()
            .map(|v| (Mfp::from(v.0), Mfp::from(v.1)))
            .collect();
        // 86x^7 + 178x^6 + 141x^5 + 52x^4 + 42x^3 + 47x^2
        let polynomial_points1 = Poly::new(vec![
            Mfp::from(86),
            Mfp::from(178),
            Mfp::from(141),
            Mfp::from(52),
            Mfp::from(42),
            Mfp::from(47),
            Mfp::from(0),
            Mfp::from(0),
        ]);

        let points2: Vec<Point> = points2
            .iter()
            .map(|v| (Mfp::from(v.0), Mfp::from(v.1)))
            .collect();
        // 68x^4 + 70x^3 + 35x^2 + 146x
        let polynomial_points2 = Poly::new(vec![
            Mfp::from(68),
            Mfp::from(70),
            Mfp::from(35),
            Mfp::from(146),
            Mfp::from(0),
        ]);

        assert_eq!(polynomial_points1, newton_interpolate(&points1));
        assert_eq!(polynomial_points2, newton_interpolate(&points2));
    }
}