// You have two coin piles containing a and b coins. On each move, you can either remove one coin from the left pile and two coins from the right pile, or two coins from the left pile and one coin from the right pile.
// Your task is to efficiently find out if you can empty both the piles.
// Input
// The first input line has an integer t: the number of tests.
// After this, there are t lines, each of which has two integers a and b: the numbers of coins in the piles.
// Output
// For each test, print "YES" if you can empty the piles and "NO" otherwise.

pub fn coin_piles(arr: Vec<u32>) -> bool {
    let a = arr[0];
    let b = arr[1];
    if (a + b) % 3 == 0 && 2 * a.min(b) >= a.max(b)  {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_piles_test1() {
        assert_eq!(coin_piles(vec![2, 1]), true);
    }

    #[test]
    fn coin_piles_test2() {
        assert_eq!(coin_piles(vec![2, 2]), false);
    }

    #[test]
    fn coin_piles_test3() {
        assert_eq!(coin_piles(vec![3, 3]), true);
    }
}
