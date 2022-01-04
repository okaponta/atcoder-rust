use proconio::{input, marker::Chars};

fn main() {
    input! {
       k:usize, mut s:Chars
    }
    if s.len() > k {
        println!("{}...", s[..k].iter().collect::<String>());
        return;
    }
    println!("{}", s.iter().collect::<String>());
}
