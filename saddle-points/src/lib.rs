fn is_saddle(i: usize, j: usize, input: &[Vec<u64>]) -> bool {
    let val = input[i][j];
    for row in input {
        if row[j] < val {
            return false;
        }
    }
    let row = &input[i];
    for &value in row {
        if value > val {
            return false;
        }
    }
    true
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (i,row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_saddle(i, j, input) {
                res.push((i, j));
            }
        }
    }
    res
}
