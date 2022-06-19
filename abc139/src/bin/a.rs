use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    println!(
        "{}",
        (0usize..3).into_iter().filter(|&i| s[i] == t[i]).count()
    );
}
