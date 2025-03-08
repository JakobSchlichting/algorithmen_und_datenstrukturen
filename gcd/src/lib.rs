pub fn gcd_brute_force(a: usize, b: usize) -> usize {
    let mut ptr = if a < b {
        a
    } else {
        b
    };
    while ptr > 0 {
        if a % ptr == 0 && b % ptr == 0 {
            break;
        }
        ptr -= 1;
    }
    ptr
}

pub fn gcd_euklid(mut a: usize, mut b: usize) -> usize {
    while a > 0 {
        if a < b { (a,b) = (b,a) }
        a = a - b;
    } 
    b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_works() {
        let result = gcd_brute_force(1025, 55);
        assert_eq!(result, 5);
    }

    #[test]
    fn euklid_works() {
        let result = gcd_euklid(1025, 55);
        assert_eq!(result, 5);
    }
}
