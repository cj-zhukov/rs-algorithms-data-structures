// Given a string, your task is to reorder its letters in such a way that it becomes a palindrome (i.e., it reads the same forwards and backwards).
// Input
// The only input line has a string of length n consisting of characters A–Z.
// Output
// Print a palindrome consisting of the characters of the original string. You may print any valid solution. If there are no solutions, print "NO SOLUTION".

use std::collections::HashMap;

pub fn palindrome_reorder(s: &str) -> Option<String> {
    let map = s.trim().chars().fold(HashMap::new(), |mut map, c| {
        map.entry(c.to_string())
            .and_modify(|n| *n += 1)
            .or_insert(1);
        map
    });

    if map.values().filter(|&e| *e % 2 != 0).count() > 1 {
        return None;
    }

    let (mut even, mut odd) = (String::new(), String::new());
    for (ch, cnt) in map {
        if cnt % 2 != 0 { odd.push_str(&ch.repeat(cnt)); }
        else { even.push_str(&ch.repeat(cnt / 2)); }
    }
    let res = format!("{}{}{}", even, odd, even.chars().rev().collect::<String>());
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_reorder_test() {
        assert_eq!(palindrome_reorder("AAAACACBA"), Some("AAACBCAAA".to_string()));
    }
}
