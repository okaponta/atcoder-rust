use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    if s.len() != 8 {
        println!("No");
        return;
    }
    if !(s[0].is_alphabetic() && s[7].is_alphabetic()) {
        println!("No");
        return;
    }
    if !(s[1].is_numeric() && s[1] != '0') {
        println!("No");
        return;
    }
    for i in 2..7 {
        if !s[i].is_numeric() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
