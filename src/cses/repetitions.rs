// You are given a DNA sequence: a string consisting of characters A, C, G, and T.
// Your task is to find the longest repetition in the sequence.
// This is a maximum-length substring containing only one type of character.
// Input
// The only input line contains a string of n characters.
// Output
// Print one integer: the length of the longest repetition.

use std::collections::HashMap;

pub fn repetitions(s: &str) -> i32 {
    let mut dna = s.chars().peekable();
    let mut cnt = 1;
    let mut max = 1;
    while let (Some(c), Some(&n)) = (dna.next(), dna.peek()) {
        if c == n {
            cnt += 1;
        } else {
            max = std::cmp::max(max, cnt);
            cnt = 1;
        }
    }
    max
}

pub fn repetitions2(input: &str) -> usize {
    let input: Vec<char> = input.chars().collect();
    let mut res = HashMap::new();
    for c in input {   
        *res.entry(c).or_insert(0) += 1;
    }
    res.values().cloned().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetitions_test() {
        assert_eq!(repetitions("ATTCGGGA"), 3);
    }

    #[test]
    fn repetitions_test2() {
        assert_eq!(repetitions2("ATTCGGGA"), 3);
    }
}
