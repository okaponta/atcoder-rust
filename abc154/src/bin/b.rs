use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    println!(
        "{}",
        (0..s.len()).into_iter().map(|_| 'x').collect::<String>()
    );
}
