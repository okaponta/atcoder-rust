use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,t:Chars
    }
    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
