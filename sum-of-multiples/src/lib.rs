pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| {
            factors
                .iter()
                .filter(|&factor| *factor != 0 && num % factor == 0)
                .peekable()
                .peek()
                .is_some()
        })
        .sum()
}
