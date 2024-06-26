pub fn reverse_words(s: String) -> String {
    let mut res = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < s.len() {
        while i < s.len() && bytes[i] == b' ' {
            i += 1;
        }

        let mut word = String::new();

        while i < s.len() && bytes[i] != b' ' {
            word.push(bytes[i] as char);
            i += 1;
        }

        if !word.is_empty() {
            res = if res.is_empty() {
                word
            } else {
                format!("{} {}", word, res)
            };
        }
    }

    res
}

pub fn reverse_words_3(s: String) -> String {
    let mut res = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < s.len() {
        let mut word = String::new();

        while i < s.len() && bytes[i] != b' ' {
            word = format!("{}{}", bytes[i] as char, word);
            i += 1;
        }

        if !res.is_empty() {
            res.push(' ');
        }

        res.push_str(&word);

        i += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("  hello       world  ".to_string()), "world hello".to_string());
        assert_eq!(reverse_words("the sky is blue".to_string()), "blue is sky the".to_string());
        assert_eq!(reverse_words("".to_string()), "".to_string());
        assert_eq!(reverse_words(" ".to_string()), "".to_string());
    }

    #[test]
    fn test_reverse_words_3() {
        assert_eq!(reverse_words_3("Let's take LeetCode contest".to_string()), "s'teL ekat edoCteeL tsetnoc".to_string());
        assert_eq!(reverse_words_3("Mr Ding".to_string()), "rM gniD".to_string());
        assert_eq!(reverse_words_3("".to_string()), "".to_string());
    }
}
