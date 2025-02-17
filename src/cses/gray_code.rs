// A Gray code is a list of all 2^n bit strings of length n, where any two successive strings differ in exactly one bit (i.e., their Hamming distance is one).
// Your task is to create a Gray code for a given length n.
// Input
// The only input line has an integer n.
// Output
// Print 2^n lines that describe the Gray code. You can print any valid solution.

pub fn gray_code(n: u32) -> Vec<String> {
    let mut res = vec![];
    (0..(0b1 << n)).for_each(|b| {
        let value = format!("{:<01$b}", (b >> 0b1) ^ b, n as usize);
        res.push(value);
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code_test() {
        assert_eq!(gray_code(2), vec!["00", "01", "11", "10"]);
    }
}
