pub fn square_of_sum(n: u32) -> u32 {
    let mut num = 0;
    for i in 1..=n {
        num += i;
    }
    (num * num) as u32
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut num = 0;
    for j in 1..=n {
        num += j * j;
    }
    num as u32
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
