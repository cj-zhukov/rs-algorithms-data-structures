// You are given a DNA sequence: a string consisting of characters A, C, G, and T.
// Your task is to find the longest repetition in the sequence.
// This is a maximum-length substring containing only one type of character.
// Input
// The only input line contains a string of n characters.
// Output
// Print one integer: the length of the longest repetition.

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

// not accurate, because it counts all elements in str
// pub fn repetitions(s: &str) -> i32 {
//     let mut map = HashMap::new();
//     for el in s.chars() {
//         match map.get_mut(&el) {
//             Some(v) => *v += 1,
//             None => {
//                 let _v = map.insert(el, 1);
//             }
//         }
//     }
//     let max_val = map
//         .iter()
//         .max_by(|a, b| a.1.cmp(&b.1))
//         .map(|(k, v)| (k, v));
//     max_val.unwrap().1.clone()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetitions_test() {
        assert_eq!(repetitions("ATTCGGGA"), 3);
    }
}
