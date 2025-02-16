// You are given an array of n integers. You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.
// On each move, you may increase the value of any element by one. What is the minimum number of moves required?
// Input
// The first input line contains an integer n: the size of the array.
// Then, the second line contains n integers x_1,x_2,\ldots,x_n: the contents of the array.
// Output
// Print the minimum number of moves.

pub fn increasing_array(_n: u64, mut arr: Vec<u64>) -> u64 {
    let mut res = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            res += arr[i - 1] - arr[i];
            arr[i] = arr[i - 1];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing_array_test() {
        assert_eq!(increasing_array(5, vec![3, 2, 5, 1, 7]), 5);
    }
}
