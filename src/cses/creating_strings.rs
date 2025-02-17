// Given a string, your task is to generate all different strings that can be created using its characters.
// Input
// The only input line has a string of length n. Each character is between a–z.
// Output
// First print an integer k: the number of strings. Then print k lines: the strings in alphabetical order.

use std::collections::BTreeSet;

pub fn creating_strings(s: String) -> Vec<String> {
    let mut res = BTreeSet::new();
    let mut chars = s.chars().collect();
    perm(0, &mut chars, &mut res);
    println!("{}", res.len());
    res.into_iter().collect()
}

fn perm(left: usize, chars: &mut Vec<char>, res: &mut BTreeSet<String>) {
    if left == chars.len() {
        res.insert(chars.iter().collect());
        return;
    }

    (left..chars.len()).for_each(|right| {
        chars.swap(left, right);
        perm(left + 1, chars, res);
        chars.swap(left, right);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_strings_test() {
        assert_eq!(creating_strings("aabac".to_string()), vec!["aaabc", "aaacb", "aabac", "aabca", "aacab", "aacba", "abaac", "abaca", "abcaa", "acaab", "acaba", "acbaa", "baaac", "baaca", "bacaa", "bcaaa", "caaab", "caaba", "cabaa", "cbaaa"]);
    }
}
