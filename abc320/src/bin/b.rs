use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    for i in (0..n).rev() {
        for j in 0..n - i {
            if (0..i).into_iter().all(|k| s[j + k] == s[j + i - k]) {
                println!("{}", i + 1);
                return;
            }
        }
    }
}
