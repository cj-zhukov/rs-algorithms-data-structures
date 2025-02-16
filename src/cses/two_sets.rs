// Your task is to divide the numbers 1,2,\ldots,n into two sets of equal sum.
// Input
// The only input line contains an integer n.
// Output
// Print "YES", if the division is possible, and "NO" otherwise.
// After this, if the division is possible, print an example of how to create the sets. First, print the number of elements in the first set followed by the elements themselves in a separate line, and then, print the second set in a similar way.

pub fn two_sets(n: i64) -> Vec<i64> {
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut sum = n * (n + 1) / 2;

    if sum % 2 != 0 { 
        return vec![];
    } else { 
        sum /= 2; 
    }

    for i in (1..=n).rev() {
        if i <= sum { 
            sum -= i; 
            a.push(i); 
        } else { 
            b.push(i) 
        }
    }
    dbg!(&a, &b);
    let mut res = vec![];
    res.push(a.len() as i64);
    res.extend(a);
    res.push(b.len() as i64);
    res.extend(b);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn two_sets_test1() {
    //     assert_eq!(two_sets(7), vec![4, 1, 2, 4, 7, 3, 3, 5, 6]);
    // }

    #[test]
    fn two_sets_test2() {
        let res = two_sets(6);
        assert_eq!(res.len(), 0);
    }
}
