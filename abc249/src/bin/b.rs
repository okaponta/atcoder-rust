use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    let a = s.iter().any(|c| c.is_uppercase());
    let b = s.iter().any(|c| c.is_lowercase());
    let len = s.len();
    s.sort();
    s.dedup();
    let c = len == s.len();
    println!("{}", if a && b && c { "Yes" } else { "No" });
}
