use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn shanks(h: usize, g: usize, modulus: usize) -> usize {
    let order = modulus - 1;
    let l = (order as f64).sqrt().ceil() as u32;
    let mut map: HashMap<u32, usize> = HashMap::new();
    let mut last = 1;
    map.insert(0, 1);
    for k in 1..l {
        last = last * g % modulus;
        map.insert(k, last);
    }
    let inverse_l: isize = inverse(g as isize, modulus).expect("There must be an inverse to it").pow(l);
    let inverse_l_pos = ((inverse_l + modulus as isize) % modulus as isize) as usize;
    let mut index = 1;
    while !map.values().any(|&val| val == (h*inverse_l_pos.pow(index) % modulus)) {
        index += 1;
    }

    let found_pair = map.iter().find(|(key, &val)| val == (h*inverse_l_pos.pow(index) % modulus)).expect("Should find the pair");
    //let found= map.values().find(|&&val| val == (h*inverse_l_pos.pow(index) % modulus)).expect("Should be some");
    (found_pair.0 + (index * l) as u32) as usize
}

fn pohlig_helman() {

}

fn inverse(number: isize, modulus: usize) -> Option<isize> {
    let m = modulus as isize;
    let (g, x, _) = egcd(number, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b%a, a);
        (g, y - (b / a) * x, x)
    }
}

fn fermat_factorize(N: usize) -> Option<usize> {
    if N % 2 == 1 {
        let mut a = (N as f64).sqrt().ceil() as usize;
        let mut b2 = a*a - N;

        while !is_square(b2) {
            a = a + 1;
            b2 = a*a - N;
        }
        Some(a - ((b2 as f64).sqrt() as usize))
    } else {
        None
    }
}

fn is_square(number: usize) -> bool {
    let n1 = (number as f64).sqrt();
    let n2 = (n1 as usize) as f64;

    n1 - n2 == 0.0
}

#[cfg(test)]
mod tests {

    use crate::{is_square, fermat_factorize, shanks, egcd, inverse};

    #[test]
    fn is_square_test() {
        assert_eq!(is_square(4), true);
        assert_eq!(is_square(3), false);
    }

    #[test]
    fn fermat_facto_test() {
        assert_eq!(fermat_factorize(5959), Some(59))
    }

    #[test]
    fn egcd_test() {
        assert_eq!(egcd(53, 44), (1, -6, 5))
    }

    #[test]
    fn inverse_mod_test() {
        assert_eq!(inverse(44, 53), Some(47));
    }

    #[test]
    fn shanks_test() {
        assert_eq!(shanks(17, 2, 53), 10);
    }
}


