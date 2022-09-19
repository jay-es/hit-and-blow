/** 数が正しいかどうか */
fn is_valid_digits(digits: &str) -> bool {
    if digits.len() != 4 {
        return false;
    }

    if digits.chars().any(|x| !x.is_numeric()) {
        return false;
    }

    let mut pre: Vec<u8> = Vec::new();
    for c in digits.bytes() {
        if pre.contains(&c) {
            return false;
        }
        pre.push(c);
    }

    true
}

#[cfg(test)]
mod is_valid_digits_test {
    use crate::is_valid_digits;

    #[test]
    fn 引数が4文字以外なら_false() {
        assert!(!is_valid_digits("123"));
        assert!(!is_valid_digits("12345"));
    }

    #[test]
    fn 数字以外が混ざっていたら_false() {
        assert!(!is_valid_digits("123a"));
        assert!(!is_valid_digits("12a3"));
    }

    #[test]
    fn 重複があったら_false() {
        assert!(!is_valid_digits("1123"));
        assert!(!is_valid_digits("1233"));
    }

    #[test]
    fn 正常系() {
        assert!(is_valid_digits("1234"));
        assert!(is_valid_digits("2468"));
    }
}
