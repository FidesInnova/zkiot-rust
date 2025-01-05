//! field opration funcitons

pub mod fmath {
    pub fn add(a: u64, b: u64, p: u64) -> u64 {
        (a + b) % p
    }

    pub fn sub(a: u64, b: u64, p: u64) -> u64 {
        match a >= b {
            true => (a - b) % p,
            false => (p - (b - a) % p) % p,
        }
    }

    pub fn mul(a: u64, b: u64, p: u64) -> u64 {
        (a * b) % p
    }

    pub fn div(a: u64, b: u64, p: u64) -> u64 {
        let b_inverse = inverse_mul(b, p);
        mul(a, b_inverse, p)
    }

    pub fn pow(a: u64, b: u64, p: u64) -> u64 {
        if p == 1 {
            return 0; // Any number mod 1 is 0
        }
        
        // Assert that (p - 1) * (p - 1) does not overflow base
        assert!((p - 1) * (p - 1) >= a);
    
        let mut result = 1;
        let mut base = a % p; // Reduce base modulo p
        let mut exponent = b;
    
        while exponent > 0 {
            if exponent % 2 == 1 { // If exponent is odd
                result = (result * base) % p; // Multiply result by base
            }
            exponent >>= 1; // Divide exponent by 2
            base = (base * base) % p; // Square the base
        }
    
        result
    }

    pub fn inverse_mul(a: u64, p: u64) -> u64 {
        pow(a, p - 2, p)
    }

    pub fn inverse_add(a: u64, p: u64) -> u64 {
        p - (a % p)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(fmath::add(5, 3, 10), 8);
        assert_eq!(fmath::add(5, 8, 10), 3); // 5 + 8 = 13 mod 10 = 3
        assert_eq!(fmath::add(10, 5, 10), 5); // 10 + 5 = 15 mod 10 = 5
    }

    #[test]
    fn test_sub() {
        assert_eq!(fmath::sub(5, 3, 10), 2);
        assert_eq!(fmath::sub(22, 8, 11), 3);
        assert_eq!(fmath::sub(3, 5, 10), 8); // 3 - 5 = -2 mod 10 = 8
        assert_eq!(fmath::sub(10, 5, 10), 5); // 10 - 5 = 5 mod 10 = 5
    }

    #[test]
    fn test_mul() {
        assert_eq!(fmath::mul(5, 3, 10), 5); // 15 mod 10 = 5
        assert_eq!(fmath::mul(4, 3, 10), 2); // 12 mod 10 = 2
        assert_eq!(fmath::mul(10, 5, 10), 0); // 50 mod 10 = 0
    }

    #[test]
    fn test_div() {
        assert_eq!(fmath::div(6, 3, 11), 2); // 6 / 3 = 2
        assert_eq!(fmath::div(13, 2, 11), 1); // 13 / 2 = 1
        assert_eq!(fmath::div(5, 3, 11), 9); // 5 / 3 = (5 * 3^-1) mod 11 = 9
    }

    #[test]
    fn test_pow() {
        assert_eq!(fmath::pow(2, 3, 10), 8); // 2^3 = 8
        assert_eq!(fmath::pow(3, 3, 10), 7); // 3^3 = 27 mod 10 = 7
        assert_eq!(fmath::pow(5, 0, 10), 1); // 5^0 = 1
        assert_eq!(fmath::pow(134, 455, 11), 10); // 134^455 mod 11 = 10
        assert_eq!(fmath::pow(1344823, 695345, 181), 26); // 1344823^695345 mod 181 = 26
    }

    #[test]
    fn test_inverse_mul() {
        assert_eq!(fmath::inverse_mul(3, 7), 5); // 3^-1 mod 7 = 5
        assert_eq!(fmath::inverse_mul(2, 7), 4); // 2^-1 mod 7 = 4
    }

    #[test]
    fn test_inverse_add() {
        assert_eq!(fmath::inverse_add(3, 10), 7); // 10 - 3 = 7
        assert_eq!(fmath::inverse_add(5, 10), 5); // 10 - 5 = 5
    }
}
