// Write a function that takes two or more arrays and returns a new array of unique values in the order of the original provided arrays.

// In other words, all values present from all arrays should be included in their original order, but with no duplicates in the final array.

// The unique numbers should be sorted by their original order, but the final array should not be sorted in numerical order.

// Check the assertion tests for examples.

pub fn unite_unique(arr: Vec<Vec<i32>>) -> Vec<i32> {
    let arr: Vec<i32> = arr.into_iter().flatten().collect();
    let mut res = vec![];
    for num in arr {
        if !res.contains(&num) {
            res.push(num);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            unite_unique([vec![1, 3, 2], vec![5, 2, 1, 4], vec![2, 1]].to_vec()),
            [1, 3, 2, 5, 4].to_vec()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            unite_unique([vec![1, 2, 3], vec![5, 2, 1]].to_vec()),
            [1, 2, 3, 5].to_vec()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            unite_unique([vec![1, 2, 3], vec![5, 2, 1, 4], vec![2, 1], vec![6, 7, 8]].to_vec()),
            [1, 2, 3, 5, 4, 6, 7, 8].to_vec()
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            unite_unique([vec![1, 3, 2], vec![5, 4], vec![5, 6]].to_vec()),
            [1, 3, 2, 5, 4, 6].to_vec()
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            unite_unique([vec![1, 3, 2, 3], vec![5, 2, 1, 4], vec![2, 1]].to_vec()),
            [1, 3, 2, 5, 4].to_vec()
        );
    }
}
