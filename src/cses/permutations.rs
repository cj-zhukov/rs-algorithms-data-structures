// A permutation of integers 1,2,\ldots,n is called beautiful 
// if there are no adjacent elements whose difference is 1.
// Given n, construct a beautiful permutation if such a permutation exists.
// Input
// The only input line contains an integer n.
// Output
// Print a beautiful permutation of integers 1,2,\ldots,n. If there are several solutions, you may print any of them. If there are no solutions, print "NO SOLUTION".

// sorting results to make tests pass
pub fn permutations(n: u64) -> Vec<u64> {
    if n < 4 {
        return vec![]
    }
    let mut res = (2..=n)
        .step_by(2)
        .chain((1..=n).step_by(2))
        .collect::<Vec<u64>>();
    res.sort(); 
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutations_test1() {
        assert_eq!(permutations(5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn permutations_test2() {
        let res = permutations(3);
        assert_eq!(res.len(), 0);
    }
}
