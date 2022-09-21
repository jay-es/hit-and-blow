mod lib;

use crate::lib::{CompareResult, Digits};

/** ユーザー入力 */
fn input() -> Digits {
    loop {
        println!("input digits");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let str = input.trim();

        match Digits::new_from_str(str) {
            Ok(v) => break v,
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    let my_digits: Digits = input();
}
