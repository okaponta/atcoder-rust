use proconio::{input, marker::Chars};

fn main() {
    input! {
        c:[Chars;3],
    }
    println!("{}{}{}", c[0][0], c[1][1], c[2][2]);
}
