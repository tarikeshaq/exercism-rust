fn is_prime(i: u32) -> bool {
    let mut j = 2;
    while j*j <= i {
        if i % j == 0 {
            return false;
        }
        j += 1;
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut i = 2;
    let mut count = 0;
    loop {
        if is_prime(i) {
            if count == n {
                return i;
            }
            count += 1;
        }
        i += 1;
    }
}
