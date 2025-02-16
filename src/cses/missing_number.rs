// You are given all numbers between 1,2,\ldots,n except one. Your task is to find the missing number
// Input
// The first input line contains an integer n.
// The second line contains n-1 numbers. Each number is distinct and between 1 and n (inclusive).
// Output
// Print the missing number.

pub fn missing_number(n: u64, arr: Vec<u64>) -> u64 {
    let sum = arr.iter().sum::<u64>();
    let res = n * (n + 1) / 2 - sum;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_number_test() {
        assert_eq!(missing_number(5, vec![2, 3, 1, 5]), 4);
    }
}