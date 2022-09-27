mod lib;

use crate::lib::{CompareResult, Digits};

/** ユーザー入力 */
fn input() -> Digits {
    loop {
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
    let mut results: Vec<String> = vec![];

    for turn in 1..10 {
        print!("\x1B[2J\x1B[1;1H"); // 画面クリア。カーソルを1行目1文字目にセット
        for s in &results {
            println!("{}", s);
        }

        println!("turn {}", turn);
        println!("please input digits");

        let my_digits: Digits = input();
        match my_digits.compare(&ans) {
            CompareResult::Same => {
                println!("you won!");
                return;
            }
            CompareResult::Diff { hit, blow } => {
                let s = format!(
                    "turn {}: \x1b[1;32m{}\x1b[0m -> hit: {}, blow: {}",
                    turn, my_digits, hit, blow
                );
                results.push(s);
            }
        }
    }

    println!("game over! the answer is {}", ans);
}
