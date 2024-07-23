

#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Add,
    Mul,
}

#[derive(Debug)]
pub struct Gate {
    pub inx_left: usize,
    pub inx_right: usize,
    pub val_left: Option<u64>,
    pub val_right: Option<u64>,
    pub gate_type: GateType,
}
impl Gate {
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



#[macro_export]
macro_rules! field {
    ($name:ident, $num:expr) => {
        pub struct P64MontConfig<const N: u64>;
        impl<const N: u64> ark_ff::MontConfig<1> for P64MontConfig<N> {
            const MODULUS: ark_ff::BigInt<1> = ark_ff::BigInt::new([N; 1]);
            const GENERATOR: ark_ff::Fp<ark_ff::MontBackend<Self, 1>, 1> = ark_ff::FpConfig::ONE;
            const TWO_ADIC_ROOT_OF_UNITY: ark_ff::Fp<ark_ff::MontBackend<Self, 1>, 1> =
                ark_ff::Fp::new(Self::MODULUS);
        }

        #[allow(warnings)]
        pub type $name = ark_ff::Fp64<ark_ff::MontBackend<P64MontConfig<$num>, 1>>;
    };
}


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


#[macro_export]
macro_rules! to_bint {
    ($var: expr) => {
        ($var).into_bigint().0[0]
    };
}


#[macro_export]
macro_rules! dsp_mat {
    ($mat: expr) => {
        for i in 0..$mat.nrows() {
            for j in 0..$mat.ncols() {
                let derr = $mat[(i, j)];
                print!(
                    "{}\t",
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
