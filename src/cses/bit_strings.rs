// Your task is to calculate the number of bit strings of length n.
// For example, if n=3, the correct answer is 8, because the possible bit strings are 000, 001, 010, 011, 100, 101, 110, and 111.
// Input
// The only input line has an integer n.
// Output
// Print the result modulo 10^9+7.

pub fn bit_strings(n: u64) -> u64 {
    const MOD: u64 = 1_000_000_007;
    (0..n).fold(1, |acc, _| acc * 2 % MOD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_strings_test() {
        assert_eq!(bit_strings(3), 8);
    }
}
