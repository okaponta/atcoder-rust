use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut count = 0;
    for i in 0..n {
        if i % 2 == 0 && s[i] == '0' {
            count += 1;
        }
        if i % 2 == 1 && s[i] == '1' {
            count += 1;
        }
    }
    println!("{}", count.min(n - count));
}
