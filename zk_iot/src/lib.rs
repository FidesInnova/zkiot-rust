//! incomplete ZKP scheme with "nalgebra" lib
pub mod parser;
pub mod utils;

use rand::prelude::SliceRandom;
extern crate nalgebra as na;
use ark_ff::{Field, PrimeField};
use na::{DMatrix, DVector};
use rand::{thread_rng, Rng};
use rustnomial::{Degree, Evaluable, FreeSizePolynomial, Polynomial, SizedPolynomial};
use utils::{Gate, GateType};
use std::collections::{HashMap, HashSet};
use std::ops::Neg;
use anyhow::{Result, anyhow};


/// Define the constant modulus for field operations.
pub const P: u64 = 181;
field!(Mfp, P);

/// Type alias for a polynomial over the `Mfp` field.
pub type Poly = Polynomial<Mfp>;

/// Type alias for a 2D point in the `Mfp` field.
pub type Point = (Mfp, Mfp);

/// Type alias for a 2D point and a value in the `Mfp` field.
pub type Point2d = (Point, Mfp);

/// Type alias for a column vector matrix with `Mfp` elements.
type ColumnVectorMatrix = nalgebra::Matrix<
    Mfp,
    nalgebra::Dyn,
    nalgebra::Const<1>,
    nalgebra::VecStorage<Mfp, nalgebra::Dyn, nalgebra::Const<1>>,
>;

/// Sets the first `t` rows of the matrix `mat` to zero.
///
/// # Parameters
/// - `mat`: Mutable reference to the matrix (`DMatrix<Mfp>`) whose rows will be modified.
/// - `t`: Number of rows to set to zero, starting from the top.
///
/// # Description
/// This function iterates over the first `t` rows of the given matrix `mat` and sets all
/// elements in these rows to zero. The number of rows affected is specified by the parameter `t`.
pub fn rows_to_zero(mat: &mut DMatrix<Mfp>, t: usize) {
    for i in 0..t {
        for j in 0..mat.ncols() {
            mat[(i, j)] = Mfp::ZERO;
        }
    }
}

/// Initializes matrices A, B, C and vector z_poly based on gate definitions.
///
/// # Parameters
/// - `gates`: A vector of `Gate` structs containing gate definitions.
/// - `ni`: Number of inputs (registers).
/// - `a_mat`: Mutable reference to matrix A to be updated.
/// - `b_mat`: Mutable reference to matrix B to be updated.
/// - `c_mat`: Mutable reference to matrix C to be updated.
/// - `z_poly`: Mutable reference to vector z_poly to be updated.
///
/// # Description
/// This function iterates through the provided `gates` vector and updates the matrices
/// A, B, and C as well as the polynomial vector `z_poly` based on the type of each gate:
/// - **Add** gates: Updates matrices and modifies `z_poly` with addition.
/// - **Mul** gates: Updates matrices and modifies `z_poly` with multiplication.
/// 
/// The matrices are populated with values according to the gate definitions, and the
/// `z_poly` vector is updated with the results of operations specified by the gates.
/// 
/// # Future Enhancements
/// Additional gate types and operations will be supported in future updates.
pub fn init(
    gates: Vec<Gate>,
    ni: usize,
    a_mat: &mut DMatrix<Mfp>,
    b_mat: &mut DMatrix<Mfp>,
    c_mat: &mut DMatrix<Mfp>,
    z_poly: &mut DVector<Mfp>,
) {
    for (i, gate) in gates.iter().enumerate() {
        let index = 1 + ni + i;
        c_mat[(index, index)] = Mfp::ONE;

        let left_val = gate.val_left.map_or(Mfp::ONE, Mfp::from);
        let right_val = gate.val_right.map_or(Mfp::ONE, Mfp::from);

        match gate.gate_type {
            GateType::Add => {
                a_mat[(index, 0)] = Mfp::ONE;

                b_mat[(index, gate.inx_left)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 2] = z_poly[i + 1] + gate.val_right.map_or(Mfp::ZERO, Mfp::from);
            }
            GateType::Mul => {
                a_mat[(index, gate.inx_left)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 2] = z_poly[i + 1] * right_val;
            }
        }
    }
}

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

    let mut numerator = Polynomial::new(vec![]);
    let mut denominator = Polynomial::new(vec![]);

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
    numerator.div_mod(&denominator).0
}


/// Performs Lagrange interpolation to find the polynomial that passes through
/// a given set of points.
///
/// # Parameters
/// - `points`: A vector of tuples where each tuple contains a point `(x_i, y_i)`
///   with `x_i` and `y_i` being coordinates in the finite field.
///
/// # Returns
/// Returns a `Poly` object representing the interpolated polynomial.
///
/// # Description
/// This function calculates the Lagrange basis polynomials for each point and
/// combines them to form the final polynomial that interpolates all given points.
/// For each point `(x_i, y_i)`, it constructs the Lagrange basis polynomial and
/// accumulates the weighted sum to form the final polynomial.
pub fn lagrange_interpolate(points: &Vec<Point>) -> Poly {
    let mut poly_res: Poly = Polynomial::new(vec![Mfp::ZERO]);

    for (x_i, y_i) in points.iter() {
        let mut poly_nume_all: Poly = Polynomial::new(vec![Mfp::ONE]);
        let mut poly_deno_all = Mfp::ONE;
        for (x_j, _) in points.iter() {
            if x_i != x_j {
                // Construct Lagrange basis polynomial for the current point
                let poly_nume: Poly =
                    Polynomial::new(vec![Mfp::ONE, Mfp::from(*x_j).neg()]);
                let poly_deno = Mfp::from(*x_i) - Mfp::from(*x_j);

                // Accumulate the numerator and denominator for the basis polynomial
                poly_nume_all *= poly_nume;
                poly_deno_all *= poly_deno;
            }
        }
        // Add the weighted basis polynomial to the result
        poly_res += Polynomial::<Mfp>::new(vec![*y_i])
            * (poly_nume_all * poly_deno_all.inverse().unwrap());
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
pub fn generate_set(ms_gen: u64, len: u64) -> Vec<Mfp> {
    (0..len).map(|i| exp_mod(ms_gen, i)).collect()
}

/// Computes the commitment of a list of polynomials using a specified degree and
/// generator.
///
/// # Parameters
/// - `o`: A vector of polynomials (`Vec<Poly>`) to commit.
/// - `d`: A degree value used in the computation.
/// - `g`: A generator value used in the computation.
///
/// # Returns
/// Returns a vector of `Mfp` values representing the commitments of the input polynomials.
///
/// # Description
/// This function computes a commitment for each polynomial in the vector `o` using
/// the given degree `d` and generator `g`. It performs the commitment calculation
/// by evaluating each polynomial and multiplying the results, adjusting based on the
/// degree and generator. If the result of the commitment is `Mfp::ONE`, it defaults to
/// the generator value `g`.
/// 
pub fn commit(o: Vec<Poly>, d: u64, g: u64) -> Vec<Mfp> {
    let mut res = vec![];

    for poly in o {
        let mut res_poly = Mfp::ONE;

        if let Degree::Num(deg) = poly.degree() {
            let mut s = Mfp::from(g);
            let d = d % (P - 1);

            for i in 0..=deg {
                let coef = poly.terms[deg - i].into_bigint().0[0];
                let value = exp_mod(s.into_bigint().0[0], coef);
                res_poly *= value;
                s = exp_mod(s.into_bigint().0[0], d);
            }
        }

        if res_poly == Mfp::ONE {
            res.push(Mfp::from(g));
        } else {
            res.push(res_poly);
        }
    }

    res
}

// Define functions to get points from a matrix based on row, column, and value modes.
define_get_points_fn!(get_points_row, row);
define_get_points_fn!(get_points_col, col);
define_get_points_fn!(get_points_val, val);


/// Generates a vector of field elements based on a given generator and parameters.
///
/// # Parameters
/// - `ms_gen`: The generator value for the finite field.
/// - `n`: The upper bound index for generating elements.
/// - `t`: The starting index for generating elements.
/// - `len`: The total length of the resulting vector.
///
/// # Returns
/// Returns a vector of `Mfp` elements. The vector starts with elements generated by raising
/// the generator `ms_gen` to powers from `t` to `n - 1`. If the vector length `len` is greater
/// than the number of generated elements, the remaining space is filled with zeros.
///
/// # Description
/// This function generates field elements using the specified generator for indices starting
/// from `t` up to `n - 1`. If the total length of the vector is greater than the number of
/// generated elements, the function appends zeros to the end of the vector to reach the
/// specified length.
///
/// # Panics
/// Panics if the calculated number of zeros to be appended is negative. This is ensured by the
/// assertion that `zeros >= 0`.
pub fn generate_set_eval(ms_gen: u64, n: usize, t: usize, len: usize) -> Vec<Mfp> {
    let mut set: Vec<Mfp> = vec![];
    for i in t..n {
        set.push(exp_mod(ms_gen, i as u64));
    }
    let zeros = len as isize - n as isize + t as isize;

    assert!(zeros >= 0);

    if zeros > 0 {
        for _ in 0..zeros {
            set.push(Mfp::ZERO);
        }
    }
    set
}

/// Creates a vector of `Point` tuples from two vectors of field elements.
///
/// # Parameters
/// - `seq`: A vector of field elements representing the y-coordinates of the points.
/// - `n`: A vector of field elements representing the x-coordinates of the points.
///
/// # Returns
/// Returns a vector of `Point` tuples, where each `Point` is a tuple of `(x, y)` coordinates
/// constructed from the corresponding elements in `n` and `seq`.
///
/// # Description
/// This function pairs elements from the `n` vector with elements from the `seq` vector to
/// form a vector of `Point` tuples. The function asserts that both vectors have the same length,
/// and then creates the vector of points by combining the corresponding elements from each vector.
///
/// # Panics
/// Panics if the lengths of `seq` and `n` are not equal. The function asserts that both vectors
/// must have the same length to ensure that each x-coordinate has a corresponding y-coordinate.
///
pub fn get_points_set(seq: &Vec<Mfp>, n: &Vec<Mfp>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    assert!(seq.len() == n.len(), "sets are not equal");

    for point in n.iter().zip(seq.iter()) {
        points.push((*point.0, *point.1));
    }
    points
}


/// Converts a column vector matrix to a vector of field elements.
///
/// # Parameters
/// - `mat`: A matrix of field elements with a single column and multiple rows.
///
/// # Returns
/// Returns a vector of `Mfp` elements, where each element is extracted from the column of the matrix.
///
/// # Description
/// This function takes a matrix with a single column and converts it into a vector of field elements.
/// It iterates over the rows of the matrix, extracting each element from the single column and adding
/// it to the resulting vector.
///
/// # Panics
/// Panics if the number of columns in the matrix is not equal to 1. The function assumes that the matrix
/// is a column vector with exactly one column.
pub fn mat_to_vec(mat: &ColumnVectorMatrix,) -> Vec<Mfp> {
    assert!(mat.ncols() < 2, "cannot convet to vec mat.ncols() < 2");

    let mut v: Vec<Mfp> = vec![];

    for i in 0..mat.nrows() {
        v.push(mat[(i, 0)]);
    }
    v
}

/// Adds a specified number of random points to a vector.
///
/// # Parameters
/// - `points`: A mutable reference to a vector of `Point` tuples where the random points will be added.
/// - `b`: The number of random points to generate and add to the vector.
/// - `set_h`: A hash set of field elements used to ensure that the generated x-coordinates are unique.
///
/// # Description
/// This function generates `b` random points where each point is a tuple `(x, y)`. The `x` coordinate is
/// selected randomly from a set of values that are not present in `set_h`, ensuring uniqueness. The `y`
/// coordinate is a random value from the field elements. The generated points are then appended to the
/// `points` vector.
pub fn push_random_points(
    points: &mut Vec<Point>,
    b: u64,
    set_h: &HashSet<Mfp>,
) {
    let mut rng = thread_rng();
    for _ in 0..b {
        let d = gen_rand_not_in_set(set_h);
        let r = Mfp::from(rng.gen_range(0..P));
        points.push((d, r));
    }
}

/// Converts a vector of field elements to a hash set.
///
/// # Parameters
/// - `vec`: A reference to a vector of `Mfp` elements.
///
/// # Returns
/// Returns a `HashSet` containing all elements from the input vector. Duplicate elements in the vector
/// will be removed in the resulting set.
///
/// # Description
/// This function takes a vector of field elements and converts it into a hash set, which removes any
/// duplicate values and allows for efficient membership checking
pub fn vec_to_hashset(vec: &Vec<Mfp>) -> HashSet<Mfp> {
    vec.iter().cloned().collect()
}


/// Generates a random field element not present in a given hash set.
///
/// # Parameters
/// - `set`: A reference to a hash set of field elements that should be excluded from the random selection.
///
/// # Returns
/// Returns a random `Mfp` element that is not in the provided hash set.
///
/// # Description
/// This function repeatedly generates random field elements until it finds one that is not in the specified
/// hash set. This ensures that the generated value is unique with respect to the given set.
///
pub fn gen_rand_not_in_set(set: &HashSet<Mfp>) -> Mfp {
    let mut rng = rand::thread_rng();
    let mut num;

    loop {
        num = Mfp::from(rng.gen_range(0..P));
        if !set.contains(&num) {
            break;
        }
    }
    num
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
    let mut vp = Polynomial::new(vec![Mfp::ONE]);

    for i in set {
        vp *= Polynomial::new(vec![Mfp::ONE, Mfp::ZERO]) - Polynomial::new(vec![*i]);
    }
    vp.trim();
    vp
}

/// Generates a random polynomial of a specified degree.
///
/// # Parameters
/// - `deg`: The degree of the polynomial to generate. The polynomial will have `deg` coefficients.
///
/// # Returns
/// Returns a `Poly` representing a polynomial with randomly generated coefficients.
///
/// # Description
/// This function creates a polynomial of the given degree with each coefficient being a random
/// field element from the finite field. The polynomial is constructed using these randomly generated
/// coefficients.
pub fn poly_gen_randomly(deg: usize) -> Poly {
    let mut rng = rand::thread_rng();
    let mut poly = vec![];

    for _ in 0..deg {
        poly.push(Mfp::from(rng.gen_range(0..P)));
    }
    
    Polynomial::new(poly)
}


/// Adds random points to the given set by pairing each element in `set_k` (starting from index `c`)
/// with a randomly chosen element from `set_h`. Returns an error if a random element cannot be chosen.
///
/// # Arguments
///
/// * `set` - A mutable vector to which the new points will be added.
/// * `set_h` - A vector of values used to pair with elements from `set_k`.
/// * `set_k` - A vector of values used to generate the new points.
/// * `c` - The starting index in `set_k` from which to begin adding points.
///
/// # Returns
///
/// A `Result` indicating success or failure, with an error message if an element from `set_h` cannot be chosen.
fn add_random_points(set: &mut Vec<(Mfp, Mfp)>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>, c: usize) -> Result<()> {
    let mut rng = thread_rng();
    for k in set_k.iter().skip(c) {
        match set_h.choose(&mut rng) {
            Some(&h) => set.push((*k, h)),
            None => return Err(anyhow!("Failed to choose a random element from set_h")),
        }
    }
    Ok(())
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
pub fn get_matrix_point_val(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>, row_k: &HashMap<Mfp, Mfp>, col_k: &HashMap<Mfp, Mfp>) -> HashMap<Mfp, Mfp> {
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
                let k   = set_k[c];
                let p2  =  val / (poly_u.eval(row_k[&k]) * poly_u.eval(col_k[&k]));
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
pub fn get_matrix_point_row(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>) -> HashMap<Mfp, Mfp> {
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
pub fn get_matrix_point_col(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>) -> HashMap<Mfp, Mfp> {
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
pub fn m_xk(num: &Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>, set_h_len: usize) -> Poly {
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
pub fn m_kx(num: &Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>, set_h_len: usize) -> Poly {
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
pub fn sigma_rkx_mkx(set_h: &Vec<Mfp>, alpha: Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>) -> Poly {
    let mut res = Poly::from(vec![Mfp::ZERO]);
    for h in set_h {
        let mut p_r_alphak = func_u(Some(alpha), Some(*h), set_h.len());
        let mut p_m_kx = m_kx(h, points_val, points_row, points_col, set_h.len());
        p_r_alphak.trim();
        p_m_kx.trim();
        res += p_r_alphak * p_m_kx;
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
pub fn sigma_rxk_mxk(set_h: &Vec<Mfp>, alpha: Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>) -> Poly {
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
/// - `k`: The `Mfp` value used to evaluate the `row`, `col`, and `val` polynomials.
/// - `row`: The row polynomial evaluated at `k`.
/// - `col`: The column polynomial evaluated at `k`.
/// - `val`: The value polynomial evaluated at `k`.
///
/// # Returns
/// Returns an `Mfp` value calculated as `eta * (nu / de)`, where `nu` is the product
/// of the Vandermonde polynomials evaluated at `beta_1` and `beta_2`, and `val` evaluated at `k`.
/// `de` is the product of differences between `beta_2` and `row(k)`, and `beta_1` and `col(k)`.
pub fn sigma_m(van_poly_vhx: &Poly, eta: &Mfp, beta_1: &Mfp, beta_2: &Mfp, k: &Mfp, row: &Poly, col: &Poly, val: &Poly) -> Mfp {
    let nu = van_poly_vhx.eval(*beta_1) * van_poly_vhx.eval(*beta_2) * val.eval(*k);
    let de = (beta_2 - &row.eval(*k)) * (beta_1 - &col.eval(*k));
    let div = nu / de;
    
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
    lagrange_interpolate(&points_li)
}



#[cfg(test)]
mod math_test {
    use super::*; 

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

        assert_eq!(exp_mod(987654321987654321, 1234567890123456789), Mfp::from(42));
        assert_eq!(exp_mod(887654448019654120, 1139562969472691009), Mfp::from(55));
        assert_eq!(exp_mod(u64::MAX, 9223372036854775807), Mfp::from(65));
    }
}