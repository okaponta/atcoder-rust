use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,t:Chars,
    }
    let eval = |s: &[char]| (0..t.len()).filter(|&i| s[i] != t[i]).count();
    println!("{}", s.windows(t.len()).map(eval).min().unwrap());
}
