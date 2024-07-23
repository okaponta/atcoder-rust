use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            println!("{} {}", i + 1, i + 2);
            return;
        }
    }
    for i in 0..n - 2 {
        if s[i] == s[i + 2] {
            println!("{} {}", i + 1, i + 3);
            return;
        }
    }
    println!("-1 -1");
}
