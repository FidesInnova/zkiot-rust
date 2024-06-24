extern crate nalgebra as na;
use na::{DMatrix};

#[derive(Clone, Copy)]
enum GateType {
    Add,
    Mult,
}

struct Gate {
    left: usize,
    right: usize,
    gate_type: GateType,
}
impl Gate {
    fn new(l: usize, r: usize, gtype: GateType) -> Self {
        Self {
            left: l,
            right: r,
            gate_type: gtype,
        }
    }
}


fn main() {
    let ng = 3; 
    let no = 2; 
    let ni = 2; 

    // TODO: using Fp
    let size = ni + ng + no + 1; 
    let mut a_matrix = DMatrix::<i32>::zeros(size, size);
    let mut b_matrix = DMatrix::<i32>::zeros(size, size);
    let mut c_matrix = DMatrix::<i32>::zeros(size, size);

    let gates = vec![
        Gate::new(1, 1, GateType::Add),
        Gate::new(1, 1, GateType::Mult),
        Gate::new(1, 1, GateType::Add),
    ];

    for (i, gate) in gates.iter().enumerate() {
        let index = 1 + ni + i;
        c_matrix[(index, index)] = 1;
        
        match gate.gate_type {
            GateType::Add => {
                a_matrix[(index, 1)] = 1;
                b_matrix[(index, 1 + gate.left)] = 1;
                b_matrix[(index, 1 + gate.right)] = 1;
            }
            GateType::Mult => {
                a_matrix[(index, 1 + gate.left)] = 1;
                b_matrix[(index, 1 + gate.right)] = 1;
            }
        }
    }

    println!("Matrix A:\n{}", a_matrix);
    println!("Matrix B:\n{}", b_matrix);
    println!("Matrix C:\n{}", c_matrix);
}
