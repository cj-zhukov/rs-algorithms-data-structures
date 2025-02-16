// Given the array arr, iterate through and remove each element starting from the first element (the 0 index)
// until the function func returns true when the iterated element is passed through it.

// Then return the rest of the array once the condition is satisfied,
// otherwise, arr should be returned as an empty array.

pub fn drop_vals<F>(arr: &mut Vec<i32>, filter: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    for (i, val) in arr.iter().enumerate() {
        if filter(val) {
            return arr[i..].to_vec();
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_vals_test() {
        let res = drop_vals(&mut vec![1, 2, 3], |&n| n > 2);
        assert_eq!(res, vec![3]);
    }
}
