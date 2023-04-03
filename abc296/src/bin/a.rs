use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    for i in 1..n {
        if s[i] == s[i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
