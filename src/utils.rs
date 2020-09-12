fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

#[cfg(test)]
mod test_digits {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }
    #[test]
    fn extract_more_digits() {
        assert_eq!(extract_digits("13-24"), ("-24", "13"));
    }
    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod test_ops {
    use super::*;

    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+2"), ("2", "+"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-10"), ("10", "-"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_op("*3"), ("3", "*"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("/4"), ("4", "/"));
    }
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c.is_whitespace(), s)
}

#[cfg(test)]
mod test_whitespace {
    use super::*;
    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }
}
