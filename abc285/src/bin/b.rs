use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    for i in 1..n {
        for j in 0..=n - i {
            if j == n - i || s[j] == s[j + i] {
                println!("{}", j);
                break;
            }
        }
    }
}
