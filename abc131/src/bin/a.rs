use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        if s.windows(2).filter(|v| v[0] == v[1]).count() > 0 {
            "Bad"
        } else {
            "Good"
        }
    );
}
