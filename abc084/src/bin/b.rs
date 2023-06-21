use proconio::{input, marker::Chars};

fn main() {
    input! {
        a:usize,
        b:usize,
        s:Chars,
    }
    for i in 0..a {
        if !s[i].is_numeric() {
            println!("No");
            return;
        }
    }
    if s[a] != '-' {
        println!("No");
        return;
    }
    for i in 0..b {
        if !s[a + 1 + i].is_numeric() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
