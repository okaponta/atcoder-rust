use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .map(|c| if c == ',' { ' ' } else { c })
            .collect::<String>()
    );
}
