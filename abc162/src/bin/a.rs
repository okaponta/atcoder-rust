use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:Chars,
    }
    println!(
        "{}",
        if n.iter().any(|d| *d == '7') {
            "Yes"
        } else {
            "No"
        }
    );
}
