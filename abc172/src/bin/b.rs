use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,t:Chars,
    }
    println!(
        "{}",
        (0..s.len()).into_iter().filter(|&i| s[i] != t[i]).count()
    );
}
