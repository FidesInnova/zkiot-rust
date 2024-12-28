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

//! Module for mathematical functions and utilities for finite field operations using the `Mfp` type and polynomials.


use ark_ff::Field;
use ark_ff::Zero;
use nalgebra::DMatrix;
use rustnomial::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::ops::Neg;
use crate::dsp_poly;
use crate::field;
use crate::json_file::ClassDataJson;
use crate::kzg;
use crate::println_dbg;
use crate::to_bint;
use rustnomial::{Polynomial, SizedPolynomial, Term};
use crate::utils::add_random_points;

pub const P: u64 = 2060801;
// pub const P: u64 = 2460193;

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


/// Divides one polynomial by another and returns the quotient and remainder.
///
/// # Parameters
/// - `a`: A reference to the dividend polynomial.
/// - `b`: A reference to the divisor polynomial.
///
/// # Returns
/// A tuple containing the quotient polynomial and the remainder polynomial.
///
/// # Panics
/// This function will panic if attempting to divide by a zero polynomial.
pub fn div_mod(a: &Poly, b: &Poly) -> (Poly, Poly) {
    let zero = Mfp::ZERO;

    let (a_first, a_degree) = match first_term(&b.terms) {
        Term::ZeroTerm => panic!("Division by zero"),
        Term::Term(coeff, deg) => (coeff, deg),
    };

    let (mut coeff, mut s_degree) = match first_term(&a.terms) {
        Term::ZeroTerm => {
            return (Polynomial::zero(), a.clone());
        }
        Term::Term(coeff, degree) => {
            if degree < a_degree {
                return (Polynomial::zero(), a.clone());
            }
            (coeff, degree)
        }
    };

    // Remainder
    let mut rem = a.terms.clone();
    let mut div = vec![zero; s_degree - a_degree + 1];
    // Offset
    let offs = s_degree;

    while s_degree >= a_degree {
        let div_res = div_mod_val(coeff, a_first);
        let scale = div_res;
        vec_sub_w_scale(&mut rem, s_degree, &b.terms, a_degree, scale);
        div[offs - s_degree] = scale;
        match first_term(&rem) {
            Term::ZeroTerm => break,
            Term::Term(coeffx, degree) => {
                coeff = coeffx;
                s_degree = degree;
            }
        }
    }

    (Poly::new(div), Poly::new(rem))
}

/// Subtracts a scaled version of the right-hand side polynomial from the left-hand side polynomial.
///
/// # Parameters
/// - `a`: A mutable slice representing the coefficients of the left-hand side polynomial.
/// - `a_deg`: The degree of the left-hand side polynomial.
/// - `b`: A slice representing the coefficients of the right-hand side polynomial.
/// - `b_deg`: The degree of the right-hand side polynomial.
/// - `b_scale`: The scaling factor to apply to the right-hand side polynomial.
fn vec_sub_w_scale(
    a: &mut [Mfp],
    a_deg: usize,
    b: &[Mfp],
    b_deg: usize,
    b_scale: Mfp,
) {
    let l = a.len() - a_deg - 1;
    for (lhs_t, rhs_t) in a[l..]
        .iter_mut()
        .zip(b[b.len() - b_deg - 1..].iter())
    {
        *lhs_t -= (*rhs_t) * b_scale;
    }
}

/// Finds the first non-zero term in a polynomial represented as a vector of coefficients.
///
/// # Parameters
/// - `poly_vec`: A slice of coefficients representing the polynomial.
///
/// # Returns
/// A `Term` representing the first non-zero term found, or `Term::ZeroTerm` if all coefficients are zero.
fn first_term(poly_vec: &[Mfp]) -> Term<Mfp> {
    for (deg, ch) in poly_vec.chunks_exact(4).enumerate() {
        for (inx, &val) in ch.iter().enumerate() {
            if !val.is_zero() {
                return Term::Term(val, poly_vec.len() - deg * 4 - inx - 1);
            }
        }
    }

    let mut inx = poly_vec.chunks_exact(4).len() * 4;
    for &val in poly_vec.chunks_exact(4).remainder().iter() {
        if !val.is_zero() {
            return Term::Term(val, poly_vec.len() - inx - 1);
        }
        inx += 1;
    }

    Term::ZeroTerm
}

/// Interpolates a polynomial that passes through a given set of points using the Newton interpolation algorithm.
///
/// # Parameters
/// - `points`: A slice of `Point` tuples, where each tuple contains an x-coordinate and a corresponding y-coordinate.
///
/// # Returns
/// A `Poly` representing the interpolating polynomial that passes through the provided points.
pub fn interpolate(points: &[Point]) -> Poly {
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
pub fn generate_set(len: u64, class_data: ClassDataJson) -> Vec<Mfp> {
    let g = to_bint!(exp_mod(class_data.g, (class_data.p - 1) / len)); // Compute the generator for set H
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
    let mut counter = 0;
    let mat_len = mat.nrows();

    let len = set_h.len();
    let mut poly_u = Poly::from(vec![Mfp::ZERO]);
    poly_u.add_term(Mfp::from(len as u64), len - 1);

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                let val = mat[(i, j)];
                assert!(set_k.get(counter).is_some());
                let k = set_k[counter];
                let div_res = div_mod_val(val, poly_u.eval(row_k[&k]) * poly_u.eval(col_k[&k]));
                let p2 = div_res;
                res.insert(set_k[counter], p2);
                counter += 1;
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
    let mut counter = 0;
    let mat_len = mat.nrows();

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert(set_k[counter], set_h[i]);
                counter += 1;
            }
        }
    }

    add_random_points(&mut res, counter, set_h, set_k).unwrap();

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

/// Retrieves the row points, column points, and computed polynomial values for non-zero elements
/// in the given matrix `mat` based on the provided sets `set_h` and `set_k`. The function
/// evaluates the necessary points and ensures the integrity of the results.
///
/// # Parameters
/// - `mat`: A reference to the matrix `mat` of type `DMatrix<Mfp>`.
/// - `set_h`: A vector of values in the finite field `Mfp`, used to identify the rows and columns.
/// - `set_k`: A vector of values in the finite field `Mfp`, used to specify the points in the matrix.
///
/// # Returns
/// Returns a tuple containing three `HashMap<Mfp, Mfp>`:
/// - The first map contains the row points.
/// - The second map contains the column points.
/// - The third map contains the computed polynomial values.
///
/// # Description
/// The function calls helper functions to gather the row points, column points, and polynomial
/// values for each non-zero element in the matrix. It asserts that the total number of row and
/// column points is twice the length of `set_k` to ensure consistency.
///
/// # Panic
/// The function will panic if the number of row and column points does not match the expected
/// count based on `set_k`.
pub fn get_matrix_points(
    mat: &DMatrix<Mfp>,
    set_h: &[Mfp],
    set_k: &[Mfp],
) -> (HashMap<Mfp, Mfp>, HashMap<Mfp, Mfp>, HashMap<Mfp, Mfp>) {
    let row_p = get_matrix_point_row(mat, &set_h, &set_k);
    // Ensure that the number of row points matches the length of set_k.
    assert_eq!(row_p.len(), set_k.len());
    
    let col_p = get_matrix_point_col(mat, &set_h, &set_k);
    // Ensure that the number of col points matches the length of set_k.
    assert_eq!(col_p.len(), set_k.len());

    let val_p = get_matrix_point_val(
        mat,
        &set_h,
        &set_k,
        &row_p,
        &col_p,
    );
    
    (row_p, col_p, val_p)
}


/// Represents the order of evaluation for polynomial computations.
///
/// The `EvalOrder` enum has two variants:
/// - `XK`: Indicates that the polynomial should be evaluated in the XK order.
/// - `KX`: Indicates that the polynomial should be evaluated in the KX order.
pub enum EvalOrder {
    XK,
    KX,
}

/// Computes a polynomial `m_k` based on the provided `points_val`, `points_row`, and `points_col`.
///
/// This function combines the functionality of the previous `m_xk` and `m_kx` functions into a single
/// function that computes a polynomial based on the specified evaluation order. The evaluation order
/// determines whether the polynomial is evaluated in the `XK` or `KX` manner.
///
/// # Parameters
/// - `num`: A reference to an `Mfp` element, used to evaluate the resulting polynomial.
/// - `points_val`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `points_row`: A `HashMap` mapping points to their corresponding row values in the matrix.
/// - `points_col`: A `HashMap` mapping points to their corresponding column values in the matrix.
/// - `set_h_len`: The length of the set `H`, which determines the degree of the polynomial.
/// - `eval_order`: An `EvalOrder` enum value that specifies the order of evaluation (either `XK` or `KX`).
///
/// # Returns
/// Returns a `Poly` representing the result of summing up the products of the evaluated polynomials.
///
/// # Description
/// This function iterates over each key-value pair `(k, val)` in `points_val`, and for each pair:
/// 1. Constructs a polynomial `poly_val` from the value `val`.
/// 2. Constructs two polynomials `poly_x` and `poly_y` using the `func_u` function, with `points_row[k]` and `points_col[k]` as inputs, respectively.
/// 3. Depending on the specified `eval_order`, it evaluates either `poly_y` at `num` (for `XK`) or `poly_x` at `num` (for `KX`).
/// 4. Multiplies the evaluated polynomial with `poly_val` and the other polynomial, then sums these products to obtain the final polynomial `poly_res`.
///
/// # Notes
/// - The final polynomial depends on the evaluation order specified by `eval_order`.
/// - This function provides a unified way to compute the polynomial interactions based on the evaluation context.
pub fn m_k(
    num: &Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
    set_h_len: usize,
    eval_order: &EvalOrder,
) -> Poly {
    let mut poly_res = Poly::from(vec![Mfp::ZERO]);

    let mut catch: HashMap<Mfp, Poly> = HashMap::new();

    eprintln!("val len: {}", points_val.len());

    for (k, h) in points_val {

        // let timer = std::time::Instant::now();
        let poly_x = catch.entry(points_row[k]).or_insert_with(|| {
            poly_func_u(None, Some(points_row[k]), set_h_len)
        }).clone();
        
        let poly_y = catch.entry(points_col[k]).or_insert_with(|| {
            poly_func_u(None, Some(points_col[k]), set_h_len)
        }).clone();
        // final_time += timer.elapsed();

        poly_res += match eval_order {
            EvalOrder::XK => {
                let res_poly_y =  poly_y.eval(*num);
                poly_x * (*h * res_poly_y)
            }
            EvalOrder::KX => {
                let res_poly_x = poly_x.eval(*num);
                poly_y * (*h * res_poly_x)
            }
        }

    }


    poly_res
}


pub fn m_k_2(
    num: &Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
    catch: &HashMap<Mfp, Poly>,
    eval_order: &EvalOrder,
) -> Poly {
    let mut poly_res = Poly::from(vec![Mfp::ZERO]);

    let mut ftime = std::time::Duration::new(0, 0);

    for (set_k_items, value) in points_val {
        // Retrieve corresponding row and column points
        let point_row = &points_row[set_k_items];
        let point_col = &points_col[set_k_items];

        // Access precomputed values from catch
        let poly_x = &catch[point_row];
        let poly_y = &catch[point_col];

        let timer = std::time::Instant::now();
        poly_res += match eval_order {
            EvalOrder::XK => {
                let res_poly_y = poly_y.eval(*num);
                let mul_poly = Poly::from(vec![(*value * res_poly_y)]);
                poly_x * &mul_poly
            }
            EvalOrder::KX => {
                let res_poly_x = poly_x.eval(*num);
                let mul_poly = Poly::from(vec![(*value * res_poly_x)]);
                poly_y * &mul_poly
            }
        };
        ftime += timer.elapsed();
    }
    eprintln!("timer - in: {:?}", ftime);
    poly_res
}

/// Computes the polynomial `u(x, y)` defined as:
/// u(x, y) = (x^n - y^n) / (x - y) = x^(n-1) + x^(n-2)y + x^(n-3)y^2 + ... + y^(n-1).
///
/// This function takes two optional inputs `x` and `y` of type `Option<Mfp>` and computes the
/// polynomial based on the given inputs:
/// - If `x` is `None` and `y` is `Some`, it generates the terms `y^k` for `k` in `[0, degree)`.
/// - If `y` is `None` and `x` is `Some`, it generates the terms `x^k` for `k` in `[0, degree)`.
/// - If both `x` and `y` are `Some`, it computes the summation of terms
///   `x^(degree - 1 - k) * y^k` for `k` in `[0, degree)`.
/// - Panics if both `x` and `y` are `None`.
///
/// # Parameters
/// - `x`: An optional value of type `Mfp` representing the base `x`.
/// - `y`: An optional value of type `Mfp` representing the base `y`.
/// - `degree`: The degree of the polynomial.
///
/// # Returns
/// A `Poly` representing the resulting polynomial.
///
/// # Panics
/// Panics if both `x` and `y` are `None`.
///
pub fn poly_func_u(x: Option<Mfp>, y: Option<Mfp>, degree: usize) -> Poly {
    match (x, y) {
        (None, Some(y)) => {
            let mut vec_poly: Vec<Mfp> = Vec::with_capacity(degree);
            let mut current_power = Mfp::ONE;
            for _ in 0..degree {
                vec_poly.push(current_power);
                current_power = current_power * y;
            }
            Poly::from(vec_poly)
        },
        (Some(x), None) => {
            let mut vec_poly: Vec<Mfp> = Vec::with_capacity(degree);
            let mut current_power = Mfp::ONE;
            for _ in 0..degree {
                vec_poly.push(current_power);
                current_power = current_power * x;
            }
            Poly::from(vec_poly)
        },
        (Some(x), Some(y)) => {
            let mut result = Mfp::ZERO;

            for k in 0..degree {
                result += exp_mod(to_bint!(x), (degree - 1 - k) as u64) * exp_mod(to_bint!(y), k as u64); 
            }

            Poly::new(vec![result])
        },
        (None, None) => panic!("Both x and y cannot be None"),
    }
}

/// Computes the polynomial sum for `sigma_rkx_mkx` based on the provided set `H`, `alpha`, and points.
///
/// # Parameters
/// - `set_h`: A reference to a vector of `Mfp` elements representing the set `H`.
/// - `alpha`: An `Mfp` element used in the polynomial computation.
/// - `points_val`: A `HashMap` mapping points to their corresponding `Mfp` values.
/// - `points_row`: A `HashMap` mapping points to their corresponding row values in the matrix.
/// - `points_col`: A `HashMap` mapping points to their corresponding column values in the matrix.
/// - `eval_order`: A reference to an `EvalOrder` enum value that specifies the order of evaluation.
///
/// # Returns
/// Returns a `Poly` representing the sum of the products of polynomials.
///
/// # Description
/// This function iterates over each element `h` in `set_h`, and for each `h`:
/// 1. Constructs a polynomial `p_r_alphak` using `func_u`, which depends on `alpha` and `h`.
/// 2. Constructs a polynomial `p_m_kx` using the `m_k` function.
/// 3. Trims the polynomials to remove leading zeros.
/// 4. Multiplies `p_r_alphak` and `p_m_kx` and sums the result into `res`.
///
/// This function is used to compute the final polynomial based on the interaction between `alpha` and `h`.
pub fn sigma_rk_mk(
    set_h: &Vec<Mfp>,
    alpha: Mfp,
    points_val: &HashMap<Mfp, Mfp>,
    points_row: &HashMap<Mfp, Mfp>,
    points_col: &HashMap<Mfp, Mfp>,
    eval_order: &EvalOrder,
    g: u64,
) -> Poly {
    let mut res = Poly::from(vec![Mfp::ZERO]);
    // eprintln!("START:");

    let mut catch: HashMap<Mfp, Poly> = HashMap::with_capacity(points_row.len() + points_col.len());
    let unique_keys: HashSet<_> = points_row.values().chain(points_col.values()).collect();

    // Precompute func_u results for unique keys
    for &key in unique_keys {
        catch.entry(key).or_insert_with(|| poly_func_u(None, Some(key), set_h.len()));
    }
    
    for h in set_h {
        let mut p_r_xk = poly_func_u(Some(alpha), Some(*h), set_h.len());

        let timer = std::time::Instant::now();
        // this part is expensive
        let mut p_m_kx = m_k_2(
            h,
            points_val,
            points_row,
            points_col,
            &catch,
            eval_order,
        );
        // ----------------------
        eprintln!("time2 : {:?}", timer.elapsed());

        p_r_xk.trim();
        p_m_kx.trim();
        
        // sigma
        res += &p_r_xk * &p_m_kx;
        // res += poly_multiply(&p_r_xk, &p_m_kx, g);
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
    let div = nu * invers_val(de);
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
    interpolate(&points_li)
}

/// Computes a pairing function based on the inputs `a`, `b`, and `g`.
///
/// This function calculates a modified pairing value by reducing `a` and `b`
/// modulo `g`, multiplying the results, and then scaling the product by a factor of 3.
///
/// # Parameters
/// - `a`: An `Mfp` value representing the first input to the pairing function.
/// - `b`: An `Mfp` value representing the second input to the pairing function.
/// - `g`: An `Mfp` value used as the modulus for reducing `a` and `b`.
///
/// # Returns
/// An `Mfp` value representing the result of the pairing computation.
pub fn e_func(a: Mfp, b: Mfp, g: Mfp) -> Mfp {
    println_dbg!("a: {a}, b: {b}");
    let a_r = Mfp::from(div_mod_val(a, g));
    let b_r = Mfp::from(div_mod_val(b, g));
    println_dbg!("a_r: {a_r}, b_r: {b_r}");
    let exp = a_r * b_r;
    Mfp::from(3) * exp
}

/// Computes the modular inverse of a given value `a` using Fermat's Little Theorem.
///
/// This function calculates the modular inverse of `a` modulo a prime `P`
/// by raising `a` to the power of `P - 2`. This is valid because if `P` is prime,
/// then the modular inverse of `a` can be computed as `a^(P-2) mod P`.
///
/// # Parameters
/// - `a`: An `Mfp` value for which the modular inverse is to be computed.
///
/// # Returns
/// An `Mfp` value representing the modular inverse of `a` modulo `P`.
pub fn invers_val(a: Mfp) -> Mfp {
    exp_mod(to_bint!(a), P - 2)
}

/// Performs modular division of two values `a` and `b` in the finite field defined by `Mfp`.
///
/// This function computes the result of `a / b` modulo a prime `P` by multiplying `a`
/// with the modular inverse of `b`. The modular inverse is calculated using the `invers_val` function.
///
/// # Parameters
/// - `a`: An `Mfp` value representing the numerator in the division.
/// - `b`: An `Mfp` value representing the denominator in the division.
///
/// # Returns
/// An `Mfp` value representing the result of the modular division `a / b` modulo `P`.
pub fn div_mod_val(a: Mfp, b: Mfp) -> Mfp {
    let numerator = a;
    let denominator = invers_val(b);
    numerator * denominator
}

/// Computes commitments for a list of polynomials using a given commitment key.
///
/// This function iterates over a slice of polynomials and computes a commitment for each polynomial
/// using the KZG commitment scheme. The results are collected in a vector and returned.
///
/// # Parameters
/// - `polys`: A slice of `Poly` representing the polynomials for which commitments are to be computed.
/// - `ck`: A reference to a vector of `Mfp` values representing the commitment key used in the KZG scheme.
///
/// # Returns
/// A vector of `Mfp` values, where each value represents the commitment for the corresponding polynomial.
pub fn compute_all_commitment(polys: &[Poly], ck: &Vec<Mfp>) -> Vec<Mfp> {
    let mut res = vec![];

    for poly in polys.iter() {
        let commitment_num = kzg::commit(&poly, &ck);
        res.push(commitment_num);
    }

    res
}

/// Multiplies two polynomials using the Number Theoretic Transform (NTT).
///
/// This function performs a polynomial multiplication in the modular arithmetic domain,
/// efficiently utilizing NTT for fast computation.
///
/// # Parameters
/// - `poly1`: A reference to the first polynomial (`Poly`).
/// - `poly2`: A reference to the second polynomial (`Poly`).
/// - `root`: The primitive root used for the NTT computation.
///
/// # Returns
/// A new polynomial (`Poly`) representing the product of the two input polynomials.
pub fn poly_multiply(poly1: &Poly, poly2: &Poly, root: u64) -> Poly {
    let p: Vec<Mfp> = ntt_multiply(
        poly1.terms.iter().map(|&v| to_bint!(v)).collect(),
        poly2.terms.iter().map(|&v| to_bint!(v)).collect(),
        P,
        root
    ).into_iter()
    .map(Mfp::from)
    .collect();

    Poly::from(p)
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
fn ntt(poly: &mut Vec<u64>, n: usize, root: u64, mod_p: u64) {
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
        let w_len = mod_exp(root, (mod_p - 1) / length as u64, mod_p);
        for i in (0..n).step_by(length) {
            let mut w = 1;
            for j in 0..length / 2 {
                let u = poly[i + j];
                let v = (poly[i + j + length / 2] * w) % mod_p;
                poly[i + j] = (u + v) % mod_p;
                poly[i + j + length / 2] = (u + mod_p - v) % mod_p;
                w = (w * w_len) % mod_p;
            }
        }
        length *= 2;
    }
}

/// Computes modular exponentiation efficiently using the method of repeated squaring.
///
/// # Parameters
/// - `base`: The base of the exponentiation.
/// - `exp`: The exponent.
/// - `mod_p`: The modulus.
///
/// # Returns
/// The result of `(base^exp) % mod_p`.
fn mod_exp(mut base: u64, mut exp: u64, mod_p: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod_p;
        }
        base = (base * base) % mod_p;
        exp /= 2;
    }
    result
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
fn ntt_multiply(poly1: Vec<u64>, poly2: Vec<u64>, mod_p: u64, root: u64) -> Vec<u64> {
    let len = poly1.len() + poly2.len() - 1;
    let n = len.next_power_of_two();

    let mut a = poly1;
    let mut b = poly2;
    a.resize(n, 0);
    b.resize(n, 0);

    ntt(&mut a, n, root, mod_p);
    ntt(&mut b, n, root, mod_p);

    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = (a[i] * b[i]) % mod_p;
    }

    let inv_n = mod_exp(n as u64, mod_p - 2, mod_p);
    ntt(&mut result, n, mod_exp(root, mod_p - 2, mod_p), mod_p);
    result.iter_mut().for_each(|x| *x = (*x * inv_n) % mod_p);

    result.into_iter().take(len).collect()
}

#[cfg(test)]
mod math_test {
    use super::*;
    #[test]
    fn test_poly_mul() {
        let a = Poly::from(vec![Mfp::ONE, Mfp::ZERO]);
        let b = Poly::from(vec![Mfp::from(2), Mfp::ONE]);

        dsp_poly!(a);
        dsp_poly!(b);

        let c = poly_multiply(&a, &b, 3);

        println!("{:?}",c);
    }

    #[test]
    fn test_div_val() {
        // Test cases
        let test_cases = vec![
            (Mfp::from(10), Mfp::from(2), Some(Mfp::from(5))),
            (Mfp::from(7), Mfp::from(1), Some(Mfp::from(7))),
            (Mfp::from(0), Mfp::from(5), Some(Mfp::from(0))),
            (Mfp::from(5), Mfp::from(0), Some(Mfp::from(0))),
            (Mfp::from(-10), Mfp::from(-2), Some(Mfp::from(5))),
            (
                Mfp::from(1_000_000_000),
                Mfp::from(1_000_000_000),
                Some(Mfp::from(1)),
            ),
        ];

        for (a, b, expected) in test_cases {
            if let Some(expected_value) = expected {
                assert_eq!(div_mod_val(a, b), expected_value)
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
            poly_func_u(Some(Mfp::from(1)), Some(Mfp::from(0)), 100)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(1)]),
            poly_func_u(Some(Mfp::from(0)), Some(Mfp::from(1)), 100)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(70)]),
            poly_func_u(Some(Mfp::from(10)), Some(Mfp::from(1)), 5)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(53)]),
            poly_func_u(Some(Mfp::from(123)), Some(Mfp::from(321)), 10)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(99)]),
            poly_func_u(Some(Mfp::from(2838193)), Some(Mfp::from(9728224)), 50)
        );
        assert_eq!(
            Poly::new(vec![Mfp::from(63)]),
            poly_func_u(Some(Mfp::from(!1)), Some(Mfp::from(!0)), 10)
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

        assert_eq!(polynomial_points1, interpolate(&points1));
        assert_eq!(polynomial_points2, interpolate(&points2));
    }
}
