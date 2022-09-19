use rand::{seq::SliceRandom, thread_rng};
use std::convert::TryInto;

const DIGITS_SIZE: usize = 4;
type Digits = [u8; DIGITS_SIZE];

/** 数の組が正しいかどうか */
pub fn is_valid_digits(digits: Digits) -> Result<(), &'static str> {
    for (i, n) in digits.iter().enumerate() {
        if *n > 9 {
            return Err("greater then 9");
        }

        // 重複チェック
        if digits[i + 1..].contains(n) {
            return Err("duplicated");
        }
    }

    Ok(())
}

#[cfg(test)]
mod is_valid_digits_test {
    use crate::is_valid_digits;

    #[test]
    fn 二桁以上の数字があったらエラー() {
        assert!(is_valid_digits([1, 2, 3, 10]).is_err());
    }

    #[test]
    fn 重複があったらエラー() {
        assert!(is_valid_digits([1, 1, 2, 3]).is_err());
        assert!(is_valid_digits([1, 2, 3, 3]).is_err());
    }

    #[test]
    fn 正常系() {
        assert!(is_valid_digits([1, 2, 3, 4]).is_ok());
        assert!(is_valid_digits([2, 4, 6, 8]).is_ok());
    }
}

/** 数の組を生成 */
pub fn generate_digits() -> Digits {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    v.shuffle(&mut rng);

    v[0..DIGITS_SIZE].try_into().unwrap()
}

#[cfg(test)]
mod generate_digits_test {
    use super::*;

    #[test]
    fn ループして異常な値がないことを確認() {
        for _ in 0..50 {
            let digits = generate_digits();
            assert!(is_valid_digits(digits).is_ok())
        }
    }

    #[test]
    fn ループして各数字が均等に現れることを確認() {
        let mut counts: Vec<u16> = vec![0; 10];

        for _ in 0..2500 {
            let digits = generate_digits();

            for n in digits.iter() {
                counts[*n as usize] += 1;
            }
        }

        for n in counts {
            assert!(900 < n && n < 1100, "{}", n);
        }
    }
}

pub fn make_digits_from_str(str: &str) -> Result<Digits, &'static str> {
    if str.len() > DIGITS_SIZE {
        return Err("too long");
    }
    if str.len() < DIGITS_SIZE {
        return Err("too short");
    }

    let mut v: Vec<u8> = vec![];

    for c in str.chars() {
        if !c.is_numeric() {
            return Err("includes non-numeric character");
        }

        v.push(c.to_string().parse().unwrap())
    }

    v.try_into().map_err(|_| "unexcepted error")
}

#[cfg(test)]
mod make_digits_from_str_test {
    use crate::make_digits_from_str;

    #[test]
    fn 入力の長さが不正ならエラー() {
        assert!(make_digits_from_str("123").is_err());
        assert!(make_digits_from_str("12345").is_err());
    }

    #[test]
    fn 数字以外が混ざっていたらエラー() {
        assert!(make_digits_from_str("123a").is_err());
        assert!(make_digits_from_str("12a3").is_err());
    }

    #[test]
    fn 正常系() {
        assert!(make_digits_from_str("1234").is_ok());
        assert!(make_digits_from_str("2468").is_ok());
        assert_eq!(make_digits_from_str("1234").unwrap(), [1, 2, 3, 4]);
        assert_eq!(make_digits_from_str("2468").unwrap(), [2, 4, 6, 8]);

        // 重複は許容
        assert!(make_digits_from_str("1111").is_ok());
    }
}
