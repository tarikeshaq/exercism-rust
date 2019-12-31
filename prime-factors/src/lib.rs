pub fn factors(n: u64) -> Vec<u64> {
    let mut current = n;
    let mut start = 2;
    let mut res = Vec::new();
    while current >= start * start {
        if current % start == 0 {
            res.push(start);
            current /= start;
        } else {
            start += 1;
        }
    }
    res.push(current);
    res
}
