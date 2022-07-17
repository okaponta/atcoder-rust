use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    if s[0] != s[1] && s[0] != s[2] {
        println!("{}", s[0]);
        return;
    }
    if s[1] != s[0] && s[1] != s[2] {
        println!("{}", s[1]);
        return;
    }
    if s[2] != s[0] && s[2] != s[1] {
        println!("{}", s[2]);
        return;
    }
    println!("-1");
}
