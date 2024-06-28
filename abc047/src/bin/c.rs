use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut num = 0;
    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            num += 1;
        }
    }
    println!("{}", num);
}
