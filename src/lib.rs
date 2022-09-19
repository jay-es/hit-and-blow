use rand::{seq::SliceRandom, thread_rng};
use std::convert::TryInto;

type DigitArray = [u8; Digits::SIZE];

pub struct Digits {
    value: DigitArray,
}

impl Digits {
    const SIZE: usize = 4;

    /** 数の配列が正しいかどうか */
    fn is_valid(digits: DigitArray) -> Result<(), &'static str> {
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

    /** ランダムな数のインスタンスを生成 */
    pub fn generate_random() -> Digits {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut rng = thread_rng();
        v.shuffle(&mut rng);

        let digits = v[0..Digits::SIZE].try_into().unwrap();
        Digits { value: digits }
    }

    fn new(digits: DigitArray) -> Result<Digits, &'static str> {
        Digits::is_valid(digits).map(|_| Digits { value: digits })
    }

    /** 文字列からインスタンスを生成 */
    pub fn new_from_str(str: &str) -> Result<Digits, &'static str> {
        if str.len() > Digits::SIZE {
            return Err("too long");
        }
        if str.len() < Digits::SIZE {
            return Err("too short");
        }

        let mut v: Vec<u8> = vec![];

        for c in str.chars() {
            if !c.is_numeric() {
                return Err("includes non-numeric character");
            }

            v.push(c.to_string().parse().unwrap())
        }

        Digits::new(v.try_into().unwrap())
    }
}

#[cfg(test)]
mod is_valid_test {
    use crate::Digits;

    #[test]
    fn 二桁以上の数字があったらエラー() {
        assert!(Digits::is_valid([1, 2, 3, 10]).is_err());
    }

    #[test]
    fn 重複があったらエラー() {
        assert!(Digits::is_valid([1, 1, 2, 3]).is_err());
        assert!(Digits::is_valid([1, 2, 3, 3]).is_err());
    }

    #[test]
    fn 正常系() {
        assert!(Digits::is_valid([1, 2, 3, 4]).is_ok());
        assert!(Digits::is_valid([2, 4, 6, 8]).is_ok());
    }
}

#[cfg(test)]
mod generate_random_test {
    use super::*;

    #[test]
    fn ループして異常な値がないことを確認() {
        for _ in 0..50 {
            let digits = Digits::generate_random();
            assert!(Digits::is_valid(digits.value).is_ok())
        }
    }

    #[test]
    fn ループして各数字が均等に現れることを確認() {
        let mut counts: Vec<u16> = vec![0; 10];

        for _ in 0..2500 {
            let digits = Digits::generate_random();

            for n in digits.value.iter() {
                counts[*n as usize] += 1;
            }
        }

        for n in counts {
            assert!(900 < n && n < 1100, "{}", n);
        }
    }
}

#[cfg(test)]
mod new_from_str_test {
    use crate::Digits;

    #[test]
    fn 入力の長さが不正ならエラー() {
        assert!(Digits::new_from_str("123").is_err());
        assert!(Digits::new_from_str("12345").is_err());
    }

    #[test]
    fn 数字以外が混ざっていたらエラー() {
        assert!(Digits::new_from_str("123a").is_err());
        assert!(Digits::new_from_str("12a3").is_err());
    }

    #[test]
    fn 重複があったらエラー() {
        assert!(Digits::new_from_str("1123").is_err());
        assert!(Digits::new_from_str("1233").is_err());
    }

    #[test]
    fn 正常系() {
        assert!(Digits::new_from_str("1234").is_ok());
        assert!(Digits::new_from_str("2468").is_ok());
        assert_eq!(Digits::new_from_str("1234").unwrap().value, [1, 2, 3, 4]);
        assert_eq!(Digits::new_from_str("2468").unwrap().value, [2, 4, 6, 8]);
    }
}
