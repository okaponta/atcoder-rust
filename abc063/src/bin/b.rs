use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    let n = s.len();
    s.sort();
    s.dedup();
    println!("{}", if n == s.len() { "yes" } else { "no" });
}
