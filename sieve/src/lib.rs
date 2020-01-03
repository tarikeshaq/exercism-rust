pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut bools = vec![false; upper_bound as usize + 1];
    (2..upper_bound).for_each(|i| {
        let mut k = 2;
        while k * i <= upper_bound {
            bools[(k * i) as usize] = true;
            k += 1;
        }
    });
    bools
        .iter()
        .enumerate()
        .filter(|(i, &val)| !val && *i != 0 && *i != 1)
        .map(|(i, _)| i as u64)
        .collect()
}
