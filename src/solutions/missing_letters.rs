// Find the missing letter in the passed letter range and return it.

// If all letters are present in the range, return undefined.

pub fn fear_not_letter(s: &str) -> Option<char> {
    let mut cur_char_code = s.chars().next()? as u8;
    let mut missing = None;
    for c in s.chars() {
        if c as u8 == cur_char_code {
            cur_char_code += 1;
        } else {
            missing = Some(cur_char_code as char);
        }
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(fear_not_letter("abce"), Some('d'));
    }

    #[test]
    fn test2() {
        assert_eq!(fear_not_letter("abcdefghjklmno"), Some('i'));
    }

    #[test]
    fn test3() {
        assert_eq!(fear_not_letter("stvwx"), Some('u'));
    }

    #[test]
    fn test4() {
        assert_eq!(fear_not_letter("bcdf"), Some('e'));
    }

    #[test]
    fn test5() {
        assert_eq!(fear_not_letter("abcdefghijklmnopqrstuvwxyz"), None);
    }
}