mod lib;

use crate::lib::{CompareResult, Digits};

/** ユーザー入力 */
fn input() -> Digits {
    loop {
        println!("input digits");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let str = input.trim();

        match Digits::new_from_str(str) {
            Ok(v) => break v,
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    let ans = Digits::generate_random();
    let mut turn: u32 = 0;

    loop {
        turn += 1;
        println!("turn: {}", turn);
        let my_digits: Digits = input();

        match my_digits.compare(&ans) {
            CompareResult::Same => {
                println!("you won!");
                return;
            }
            CompareResult::Diff { hit, blow } => {
                println!("-> hit: {}, blow: {}", hit, blow);
            }
        }
    }
}
