// Your task is to count for k=1,2,\ldots,n 
// the number of ways two knights can be placed on a k \times k chessboard 
// so that they do not attack each other.
// Input
// The only input line contains an integer n.
// Output
// Print n integers: the results.

pub fn two_knights(n: i64) -> Vec<i64> {
    let mut res = vec![];
    (1..=n).for_each(|k| {
        let value = (k.pow(2) * (k.pow(2) - 1) / 2) - (4 * (k - 2) * (k - 1));
        res.push(value);
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_knights_test() {
        assert_eq!(two_knights(8), vec![0, 6, 28, 96, 252, 550, 1056, 1848]);
    }
}
