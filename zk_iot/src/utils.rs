//! Utility functions and structures for gate definitions, matrix operations, and polynomial encoding.

use rand::prelude::SliceRandom;
use rustnomial::SizedPolynomial;
use std::collections::HashMap;
use std::collections::HashSet;
use anyhow::{Result, anyhow};
use nalgebra::DMatrix;
use rand::thread_rng;
use ark_ff::Field;
use sha2::Digest;
use rand::Rng;





use crate::define_get_points_fn;
use crate::get_val;

use crate::math::lagrange_interpolate;
use crate::math::Point;
use crate::math::Poly;
use crate::math::Mfp;
use crate::math::P;

/// Represents the type of a gate.
///
/// This enum defines the possible types of gates,
/// specifically addition and multiplication gates.
#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Add,
    Mul,
    Ld
}

/// Represents a gate with its parameters.
///
/// # Fields
/// - `inx_left`: The index of the left input of the gate.
/// - `inx_right`: The index of the right input of the gate.
/// - `val_left`: Optional value for the left input, if provided.
/// - `val_right`: Optional value for the right input, if provided.
/// - `gate_type`: The type of the gate, which can be either an addition or multiplication gate.
///
/// # Description
/// This struct is used to define a gate. It includes the indices for the
/// left and right inputs, optional values for these inputs, and the type of gate being used.
#[derive(Debug)]
pub struct Gate {
    pub inx_left: usize,
    pub inx_right: usize,
    pub val_left: Option<u64>,
    pub val_right: Option<u64>,
    pub gate_type: GateType,
}

impl Gate {
    /// Creates a new instance of a `Gate`.
    ///
    /// # Parameters
    /// - `l`: The index of the left input.
    /// - `r`: The index of the right input.
    /// - `val_left`: Optional value for the left input.
    /// - `val_right`: Optional value for the right input.
    /// - `gtype`: The type of gate (addition or multiplication).
    ///
    /// # Returns
    /// Returns a `Gate` instance with the specified parameters.
    ///
    /// # Description
    /// This constructor method initializes a `Gate` with the provided indices, optional
    /// values, and gate type.
    pub fn new(
        l: usize,
        r: usize,
        val_left: Option<u64>,
        val_right: Option<u64>,
        gtype: GateType,
    ) -> Self {
        Self {
            inx_left: l,
            inx_right: r,
            val_left,
            val_right,
            gate_type: gtype,
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
    z_poly: &mut DMatrix<Mfp>,
) {
    let mut index = 1 + ni;
    let mut counter = 0;
    let mut ld_counter = 0;
    for (i, gate) in gates.iter().enumerate() {
        index = 1 + ni + counter;
        c_mat[(index, index)] = Mfp::ONE;

        let left_val = gate.val_left.map_or(Mfp::ONE, Mfp::from);
        let right_val = gate.val_right.map_or(Mfp::ONE, Mfp::from);

        match gate.gate_type {
            GateType::Add => {
                a_mat[(index, 0)] = Mfp::ONE;

                b_mat[(index, gate.inx_left - ld_counter)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 1] = z_poly[i] + gate.val_right.map_or(Mfp::ZERO, Mfp::from);
            }
            GateType::Mul => {
                a_mat[(index, gate.inx_left - ld_counter)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 1] = z_poly[i] * right_val;
            }
            GateType::Ld => {
                z_poly[i + 1] = right_val;
                ld_counter += 1;
                continue;
            },
        }
        counter += 1;

    }
}


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



// Define functions to get points from a matrix based on row, column, and value modes.
define_get_points_fn!(get_points_row, row);
define_get_points_fn!(get_points_col, col);
define_get_points_fn!(get_points_val, val);



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
pub fn get_points_set(seq: &[Mfp], n: &[Mfp]) -> Vec<Point> {
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
pub fn mat_to_vec(mat: &DMatrix<Mfp>) -> Vec<Mfp> {
    assert!(mat.ncols() == 1, "cannot convet to vec mat.ncols() == 1");

    let mut v = vec![];

    for i in 0..mat.nrows() {
        v.push(mat[(i, 0)]);
    }
    v
}



/// Converts a vector of `Mfp` elements into a `HashSet` of `Mfp`.
///
/// # Parameters
/// - `set`: A reference to a vector of `Mfp` elements.
///
/// # Returns
/// Returns a `HashSet` containing the unique elements from the input vector.
///
/// # Description
/// This function iterates through the given vector and collects its elements
/// into a `HashSet`, which removes any duplicates and allows for efficient
/// membership checking.
pub fn vec_to_set(set: &[Mfp]) -> HashSet<Mfp> {
    set.iter().copied().collect()
}


/// Generates a random field element not present in a given set.
///
/// # Parameters
/// - `set`: A reference to a set of field elements that should be excluded from the random selection.
///
/// # Returns
/// Returns a random `Mfp` element that is not in the provided set.
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
    
    Poly::new(poly)
}




/// Adds random points to the given `points` HashMap by pairing each element in `set_k` 
/// (starting from index `c`) with a randomly chosen element from `set_h`. 
/// Returns an error if a random element cannot be chosen from `set_h`.
///
/// # Arguments
///
/// * `points` - A mutable HashMap where new points will be added, with keys from `set_k` 
///   and values chosen randomly from `set_h`.
/// * `c` - The starting index in `set_k` from which to begin adding points.
/// * `set_h` - A slice of values used to pair with elements from `set_k`.
/// * `set_k` - A slice of values used to generate the new points.
///
/// # Returns
///
/// A `Result<()>` indicating success or failure. If successful, it returns `Ok(())`. 
/// If an error occurs while choosing a random element from `set_h`, it returns an error.
pub fn add_random_points(
    points: &mut HashMap<Mfp, Mfp>, 
    c: usize, 
    set_h: &[Mfp], 
    set_k: &[Mfp]
) -> Result<()> {
    let mut rng = rand::thread_rng();

    for i in c..set_k.len() {
        let rand_h = set_h.choose(&mut rng).ok_or(anyhow!("Failed to choose a random element from set_h"))?;
        // points.insert(set_k[i], *rand_h);
    }


    Ok(())
}

pub fn print_hashmap(points: &HashMap<Mfp, Mfp>, set_k: &[Mfp]) {
    for k in set_k.iter() {
        if let Some(val) = points.get(k) {
            println!("{} = {}", k, val);
        } else {
            println!("{} = None", k);
        }
    }
}

/// Encodes a matrix into three polynomials: row, column, and value polynomials.
///
/// # Parameters
/// - `matrix`: A reference to a `DMatrix<Mfp>`, representing the matrix to be encoded.
/// - `set_h`: A reference to a vector of `Mfp` elements, representing the set H used in the encoding process.
/// - `set_k`: A reference to a vector of `Mfp` elements, representing the set K used in the encoding process.
///
/// # Returns
/// Returns a vector containing three polynomials: the row polynomial, the column polynomial, and the value polynomial.
///
/// # Description
/// This function encodes a given matrix into three separate polynomials:
/// 1. The row polynomial is obtained by performing Lagrange interpolation on the points corresponding to the rows of the matrix.
/// 2. The column polynomial is obtained by performing Lagrange interpolation on the points corresponding to the columns of the matrix.
/// 3. The value polynomial is obtained by performing Lagrange interpolation on the points corresponding to the non-zero values in the matrix.
pub fn encode_matrix_m(matrix: &DMatrix<Mfp>, set_h: &[Mfp], set_k: &[Mfp]) -> Vec<Poly> {
    let points  = get_points_row(matrix, set_h, set_k);   
    let row     = lagrange_interpolate(&points);                                        
    let points  = get_points_col(matrix, set_h, set_k);  
    let col     = lagrange_interpolate(&points);                 
    let points  = get_points_val(matrix, set_h, set_k);
    let val     = lagrange_interpolate(&points);

    // println!("lag row:");
    // dsp_poly!(row);
    // println!("lag col:");
    // dsp_poly!(col);
    // println!("lag val:");
    // dsp_poly!(val);
    
    vec![row, col, val]
}


/// Defines a field configuration and type alias for a given modulus.
///
/// This macro generates a field configuration for a Montgomery representation of a prime field
/// with a specified modulus. It creates a struct implementing the `MontConfig` trait and defines
/// a type alias for a field element.
///
/// # Parameters
/// - `$name`: The name of the type alias for the field element.
/// - `$num`: The modulus of the field, which should be a `u64` constant.
///
/// # Description
/// The macro defines a `P64MontConfig` struct with a constant modulus and a Montgomery representation
/// of the field. It implements the `ark_ff::MontConfig` trait to configure the field with the provided
/// modulus and initializes the generator and two-adic root of unity. The macro then creates a type alias
/// `$name` for the field element using `ark_ff::Fp64` with the defined configuration.
///
/// # Example
/// ```
/// use zk_iot::field;
/// field!(MyField, 1234567890123456789);
/// let x: MyField = MyField::from(10);
/// ```
#[macro_export]
macro_rules! field {
    ($name:ident, $num:expr) => {
        pub struct P64MontConfig<const N: u64>;
        impl<const N: u64> ark_ff::MontConfig<1> for P64MontConfig<N> {
        const MODULUS: ark_ff::BigInt<1> = ark_ff::BigInt::new([N; 1]);
        const GENERATOR: ark_ff::Fp<ark_ff::MontBackend<Self, 1>, 1> 
        = <ark_ff::Fp64<ark_ff::MontBackend<P64MontConfig<N>, 1>> as ark_ff::Field>::ONE;
        const TWO_ADIC_ROOT_OF_UNITY: ark_ff::Fp<ark_ff::MontBackend<Self, 1>, 1> =
            ark_ff::Fp::new(Self::MODULUS);
}
        #[allow(warnings)]
        pub type $name = ark_ff::Fp64<ark_ff::MontBackend<P64MontConfig<$num>, 1>>;
    };
}


/// Retrieves a value based on the specified mode and input parameters.
///
/// # Parameters
/// - `row`: Retrieves the value from vector `h` using the row index `$i`.
/// - `col`: Retrieves the value from vector `h` using the column index `$j`.
/// - `val`: Retrieves the value from the matrix `$mat` at position `($i, $j)`.
#[macro_export]
macro_rules! get_val {
    (row, $h:expr, $_:expr, $i:expr, $j:expr) => {
        $h[$i]
    };
    (col, $h:expr, $_:expr, $i:expr, $j:expr)=> {
        $h[$j]
    };
    (val, $_:expr, $mat:expr, $i:expr, $j:expr) => {
        $mat[($i, $j)]
    };
}

/// Defines a function for extracting points from a matrix based on a specified mode.
///
/// # Parameters
/// - `$name`: The name of the function to be defined (e.g., `get_points_row`).
/// - `$mode`: The mode to use for extracting values (e.g., `row`, `col`, `val`).
///
/// # Description
/// This macro generates a function that iterates over the non-zero elements of a matrix
/// and collects points based on the specified mode. The generated function takes three parameters:
/// - `mat`: A matrix (`DMatrix<Mfp>`) from which to extract points.
/// - `h`: A vector of `Mfp` values used in conjunction with the matrix to determine the point values.
/// - `k`: A vector of `Mfp` values used as the x-coordinates of the points.
///
/// The macro generates functions like `get_points_row`, `get_points_col`, and `get_points_val`,
/// each tailored to extract points based on the mode (`row`, `col`, or `val`) specified during macro invocation.
#[macro_export]
macro_rules! define_get_points_fn {
    ($name:ident, $mode:ident) => {

        #[allow(unused_variables)]
        pub fn $name(
            mat: &DMatrix<Mfp>,
            h: &[Mfp],
            k: &[Mfp],
        ) -> Vec<(Mfp, Mfp)> {
            let mut points: Vec<(Mfp, Mfp)> = vec![];
            let mut c = 0;

            for i in 0..mat.nrows() {
                for j in 0..mat.ncols() {
                    if mat[(i, j)] != Mfp::ZERO {
                        let value = get_val!($mode, h, mat, i, j);
                        points.push((k[c], value));
                        c += 1;
                    }
                }
            }
            points
        }
    };
}

/// Converts a field element to a `BigInt` representation.
///
/// # Parameters
/// - `$var`: The field element to be converted, which is expected to implement the `IntoBigInt` trait.
///
/// # Returns
/// Returns the `BigInt` representation of the given field element. This conversion extracts the integer
/// value from the field element's underlying representation.
///
/// # Description
/// This macro converts a field element into its `BigInt` representation by calling the `into_bigint`
/// method on the element and then accessing the underlying integer. This is useful for operations that
/// require the integer value of a field element.
///
/// # Example
/// ```
/// use zk_iot::field;
/// use zk_iot::to_bint;
/// use ark_ff::PrimeField; // for into_bigint()
/// 
/// field!(MyField, 1234567890123456789);
/// let x: MyField = MyField::from(10);
/// 
/// let big_int = to_bint!(x);
/// assert_eq!(big_int, x.into_bigint().0[0]);
/// ```
#[macro_export]
macro_rules! to_bint {
    ($var: expr) => {
        ark_ff::PrimeField::into_bigint($var).0[0]
    };
}

/// Displays the contents of a matrix.
///
/// # Parameters
/// - `$mat`: A reference to the matrix to be displayed. The matrix should implement indexing
///   via `(i, j)` to access elements.
///
/// # Description
/// This macro iterates over the rows and columns of the provided matrix, printing each element.
#[macro_export]
macro_rules! dsp_mat {
    ($mat: expr) => {
        for i in 0..$mat.nrows() {
            for j in 0..$mat.ncols() {
                let derr = $mat[(i, j)];
                print!(
                    "{:<10}",
                    if derr == Mfp::ZERO {
                        "0".to_owned()
                    } else {
                        format!("{}", derr)
                    }
                );
            }
            println!();
        }
        println!();
    };
}


/// Converts a vector to a formatted string with elements separated by commas.
///
/// # Parameters
/// - `$ve`: A reference to the vector to be converted to a string. The vector should implement
///   the `Display` trait for its elements.
///
/// # Returns
/// Returns a string containing the vector elements separated by commas, with no trailing comma
/// at the end.
///
/// # Description
/// This macro iterates over the elements of the provided vector, concatenating them into a
/// comma-separated string. The resulting string is useful for displaying
/// of the vector.
#[macro_export]
macro_rules! dsp_vec {
    ($ve: expr) => {{
        let mut result = String::new();

        for (i, x) in $ve.iter().enumerate() {
            if i == $ve.len() - 1 {
                result.push_str(&format!("{}", x));
            } else {
                result.push_str(&format!("{}, ", x));
            }
        }
        
        result
    }};
}

/// Displays a polynomial in human-readable format.
///
/// # Parameters
/// - `$poly`: A reference to the polynomial to be displayed. The polynomial should implement the
///   `Clone`, `Degree`, and `SizedPolynomial` traits, and its terms should implement `Display`.
///
/// # Description
/// This macro formats the given polynomial as a string, showing each term in the format `ax^b`
/// where `a` is the coefficient and `b` is the exponent.
#[macro_export]
macro_rules! dsp_poly {
    ($poly:expr) => {{
        use std::io::Write;
        use rustnomial::{SizedPolynomial, Degree};

        let mut result = String::new();
        let mut poly = $poly.clone();
        poly.trim();
        if let Degree::Num(deg) = poly.degree() {
            for (i, term) in poly.terms.iter().enumerate() {
                if *term != Mfp::ZERO && i < deg + 1 {
                    if i != 0 {
                        result.push_str(" + ");
                    }
                    if *term == Mfp::ONE && deg > i {
                        result.push_str(&format!("x^{}", deg - i));
                    } else if deg == i {
                        result.push_str(&format!("{}", term));
                    } else if deg == i + 1 {
                        result.push_str(&format!("{}x", term));
                    } else if deg > i {
                        result.push_str(&format!("{}x^{}", term, deg - i));
                    }
                }
            } 
        }
        
        println!("{result}\n");
    }};
}

pub fn sha2_hash(input: &str) -> u64 {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    u64::from_le_bytes([
        result[31], result[30], result[29], result[28],
        result[27], result[26], result[25], result[24],
    ])
}


pub fn concat_polys(polys: &[&Poly]) -> Vec<Mfp> {
    let mut result = vec![];

    for poly in polys {
        result.extend(poly.terms_as_vec().iter().map(|v| v.0)); 
    }

    result
}