use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    s.sort();
    let ans = s.into_iter().collect::<String>();
    println!("{}", if ans == "abc" { "Yes" } else { "No" });
}
