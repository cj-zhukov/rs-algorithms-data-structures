// Return an English translated sentence of the passed binary string.
// The binary string will be space separated.

pub fn binary_agent(s: &str) -> String {
    let mut res = String::new();
    for byte in s.split(' ').map(|x| u8::from_str_radix(x, 2).unwrap()) {
        res.push(byte as char);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_agents_test() {
        assert_eq!(binary_agent("1100110 1101111 1101111 100000 1100010 1100001 1110010 100000 1100010 1100001 1111010"), "foo bar baz");
    }
}
