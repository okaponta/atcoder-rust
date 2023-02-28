use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter().enumerate().find(|(_, c)| *c < 'a').unwrap().0 + 1
    );
}
