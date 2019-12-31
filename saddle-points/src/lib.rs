fn is_saddle(i: usize, j: usize, input: &[Vec<u64>]) -> bool {
    let value = input[i][j];
    input.iter().all(|row| row[j] >= value) && input[i].iter().all(|&val| val <= value)
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_saddle(i, j, input) {
                res.push((i, j));
            }
        }
    }
    res
}
