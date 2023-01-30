use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .fold(0, |s, c| if c == '+' { s + 1 } else { s - 1 })
    );
}
