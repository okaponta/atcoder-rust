use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
        mut t:Chars,
    }
    s.sort();
    t.sort();
    t.reverse();
    println!("{}", if s < t { "Yes" } else { "No" });
}
