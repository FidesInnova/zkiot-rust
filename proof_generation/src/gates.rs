// These gates are hardcoded
// In the proof generation phase, the gates are processed through another 
// method that is not implemented in the Rust code

vec![
    Gate {
        val_left: None,
        val_right: Some(
            4,
        ),
        reg_left: 0,
        reg_right: 0,
        gate_type: Ld,
    },
    Gate {
        val_left: None,
        val_right: Some(
            5,
        ),
        reg_left: 0,
        reg_right: 0,
        gate_type: Mul,
    },
    Gate {
        val_left: None,
        val_right: Some(
            11,
        ),
        reg_left: 0,
        reg_right: 0,
        gate_type: Add,
    },
    Gate {
        val_left: None,
        val_right: Some(
            26,
        ),
        reg_left: 0,
        reg_right: 0,
        gate_type: Mul,
    },
]