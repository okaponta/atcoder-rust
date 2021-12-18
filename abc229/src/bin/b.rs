use proconio::{input, marker::Chars};

fn main() {
    input! {
       a:Chars,b:Chars,
    }
    let a_rev = a.iter().rev().collect::<Vec<&char>>();
    let b_rev = b.iter().rev().collect::<Vec<&char>>();
    for i in 0..a.len().min(b.len()) {
        if a_rev[i].to_digit(10).unwrap() + b_rev[i].to_digit(10).unwrap() > 9 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
