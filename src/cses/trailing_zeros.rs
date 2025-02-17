// Your task is to calculate the number of trailing zeros in the factorial n!.
// For example, 20!=2432902008176640000 and it has 4 trailing zeros.
// Input
// The only input line has an integer n.
// Output
// Print the number of trailing zeros in n!.

pub fn trailing_zeros(n: u64) -> u64 {
    let mut res = 0;
    let mut n = n;
    while n > 0 {
        res += n/5;
        n /= 5;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_zeros_test() {
        assert_eq!(trailing_zeros(20), 4);
    }
}
