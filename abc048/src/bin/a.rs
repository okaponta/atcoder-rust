use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars;3],
    }
    println!("A{}C", s[1][0]);
}
