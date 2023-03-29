use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .fold(700, |s, c| if c == 'o' { s + 100 } else { s })
    );
}
