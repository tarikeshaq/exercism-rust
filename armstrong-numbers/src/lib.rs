pub fn is_armstrong_number(num: u32) -> bool {
    let mut current = num;
    let num_of_digits = f32::log10(num as f32) as u32 + 1;
    let mut sum = 0;
    while current != 0 {
        let digit = current % 10;
        sum += u32::pow(digit, num_of_digits);
        current /= 10;
    }
    sum == num
}
