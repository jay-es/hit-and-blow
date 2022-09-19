use rand::{thread_rng, Rng};

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

fn generate_digits() -> String {
    let mut digits = String::new();

    for n in 0..10 {
        let idx = thread_rng().gen_range(0..=n);
        digits.insert_str(idx, &n.to_string());
    }

    (digits[0..4]).to_string()
}

#[cfg(test)]
mod generate_digits {
    use super::*;

    #[test]
    fn ループして異常な値がないことを確認() {
        for _ in 0..50 {
            let digits = generate_digits();
            assert!(is_valid_digits(&digits), "{}", digits)
        }
    }

    #[test]
    fn ループして各数字が均等に現れることを確認() {
        let mut counts: Vec<u16> = vec![0; 10];

        for _ in 0..2500 {
            let digits = generate_digits();

            for c in digits.chars() {
                if let Ok(idx) = c.to_string().parse::<usize>() {
                    counts[idx] += 1;
                }
            }
        }

        for n in counts {
            assert!(900 < n && n < 1100, "{}", n);
        }
    }
}
