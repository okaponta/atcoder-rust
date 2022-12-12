use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .map(|c| if c == '1' { '9' } else { '1' })
            .collect::<String>()
    );
}
