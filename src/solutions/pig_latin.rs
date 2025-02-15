// Pig Latin is a way of altering English Words. The rules are as follows:

// - If a word begins with a consonant, take the first consonant or consonant cluster,
// move it to the end of the word, and add ay to it.

// - If a word begins with a vowel, just add way at the end.

// Translate the provided string to Pig Latin. Input strings are guaranteed to be English words in all lowercase.

use regex::Regex;

pub fn translate_pig_latin(s: String) -> String {
    let re = Regex::new(r"^[aeiou]+").unwrap();
    let mut res = String::new();
    // If s starts with vowel, push to result string and add "way" at the end
    if re.is_match(&s) {
        let new_str = format!("{}way", &s);
        res.push_str(&new_str);
    } else {
        // If starting with consonant(s) then taking the consonant(s) and putting them at the end,
        // then adding "ay"
        let re = Regex::new(r"(^[^aeiou]+)(\w*)").unwrap();
        res = re.replace(&s, "${2}${1}ay").to_string();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            translate_pig_latin("california".to_string()),
            "aliforniacay"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            translate_pig_latin("paragraphs".to_string()),
            "aragraphspay"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(translate_pig_latin("glove".to_string()), "oveglay");
    }

    #[test]
    fn test4() {
        assert_eq!(translate_pig_latin("algorithm".to_string()), "algorithmway");
    }
}
