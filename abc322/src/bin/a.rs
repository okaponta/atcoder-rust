use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
