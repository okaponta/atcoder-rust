use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let r = s.iter().position(|&c| c == 'R').unwrap();
    let m = s.iter().position(|&c| c == 'M').unwrap();
    println!("{}", if r < m { "Yes" } else { "No" });
}
