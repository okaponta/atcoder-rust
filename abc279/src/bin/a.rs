use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .fold(0, |a, c| a + if c == 'v' { 1 } else { 2 })
    );
}
