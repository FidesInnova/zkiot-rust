/// Represents the type of a gate in a circuit.
///
/// This enum defines the possible types of gates that can be used in a circuit,
/// specifically addition and multiplication gates.
#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Add,
    Mul,
}

/// Represents a gate with its parameters in a circuit.
///
/// # Fields
/// - `inx_left`: The index of the left input of the gate.
/// - `inx_right`: The index of the right input of the gate.
/// - `val_left`: Optional value for the left input, if provided.
/// - `val_right`: Optional value for the right input, if provided.
/// - `gate_type`: The type of the gate, which can be either an addition or multiplication gate.
///
/// # Description
/// This struct is used to define a gate in a circuit. It includes the indices for the
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
            h: &Vec<Mfp>,
            k: &Vec<Mfp>,
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
/// comma-separated string. The resulting string is useful for displaying or logging the contents
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
