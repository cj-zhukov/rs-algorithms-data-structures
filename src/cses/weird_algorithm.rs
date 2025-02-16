// Consider an algorithm that takes as input a positive integer n.
// If n is even, the algorithm divides it by two, and if n is odd, 
// the algorithm multiplies it by three and adds one.
// The algorithm repeats this, until n is one.

pub fn weird_algorithm(input: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut n = input;
    res.push(n);
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
            res.push(n);
        } else {
            n = n * 3 + 1;
            res.push(n);
        }
        dbg!(n);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weird_algorithm_test() {
        assert_eq!(weird_algorithm(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
    }
}
