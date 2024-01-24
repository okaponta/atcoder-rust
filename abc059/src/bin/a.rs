use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:[Chars;3],
    }
    println!(
        "{}",
        s.iter()
            .map(|s| s[0].to_ascii_uppercase())
            .collect::<String>()
    );
}
