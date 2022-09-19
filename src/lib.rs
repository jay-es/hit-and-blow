use rand::{seq::SliceRandom, thread_rng};
use std::convert::TryInto;

type Digits = [u8; 4];

/** 数の組が正しいかどうか */
pub fn is_valid_digits(digits: Digits) -> bool {
    for (i, n) in digits.iter().enumerate() {
        if *n > 9 {
            return false;
        }

        // 重複チェック
        if digits[i + 1..].contains(n) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod is_valid_digits_test {
    use crate::is_valid_digits;

    #[test]
    fn 二桁以上の数字があったら_false() {
        assert!(!is_valid_digits([1, 2, 3, 10]));
    }

    #[test]
    fn 重複があったら_false() {
        assert!(!is_valid_digits([1, 1, 2, 3]));
        assert!(!is_valid_digits([1, 2, 3, 3]));
    }

    #[test]
    fn 正常系() {
        assert!(is_valid_digits([1, 2, 3, 4]));
        assert!(is_valid_digits([2, 4, 6, 8]));
    }
}

/** 数の組を生成 */
pub fn generate_digits() -> Digits {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    v.shuffle(&mut rng);

    v[0..4].try_into().unwrap()
}

#[cfg(test)]
mod generate_digits {
    use super::*;

    #[test]
    fn ループして異常な値がないことを確認() {
        for _ in 0..50 {
            let digits = generate_digits();
            assert!(is_valid_digits(digits))
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
