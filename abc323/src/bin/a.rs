use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    if (0..8).into_iter().all(|i| s[i * 2 + 1] == '0') {
        println!("Yes");
    } else {
        println!("No");
    }
}
