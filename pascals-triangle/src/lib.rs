pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res: Vec<Vec<u32>> = Vec::new();
        for i in 0..self.row_count {
            let mut curr = Vec::new();
            if i == 0 {
                curr.push(1);
            } else {
                let prev = res.last().unwrap();
                for j in 0..=i {
                    if j == 0 {
                        curr.push(1);
                    } else {
                        let left = prev.get(j as usize - 1);
                        let right = prev.get(j as usize);
                        if left.is_none() || right.is_none() {
                            curr.push(1);
                        } else {
                            curr.push(left.unwrap() + right.unwrap());
                        }
                    }
                }
            }
            res.push(curr);
        }

        res
    }
}
