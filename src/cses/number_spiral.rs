// A number spiral is an infinite grid whose upper-left square has number 1. Here are the first five layers of the spiral: 
// Your task is to find out the number in row y and column x.
// Input
// The first input line contains an integer t: the number of tests.
// After this, there are t lines, each containing integers y and x.
// Output
// For each test, print the number in row y and column x.

pub fn number_spiral(arr: Vec<(u64, u64)>) -> Vec<u64> {
    let mut result = vec![];
    for (r, c) in arr {
        let res = if r > c {
            if r % 2 == 0 { 
                r.pow(2) - c + 1
            }
            else { 
                (r - 1).pow(2) + c
            }
        } else {
            if c % 2 == 0 { 
                (c - 1).pow(2) + r
            }
            else { 
                c.pow(2) - r + 1
            }
        };
        result.push(res);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_spiral_test() {
        assert_eq!(number_spiral(vec![(2, 3), (1, 1), (4, 2)]), vec![8, 1, 15]);
    }
}
