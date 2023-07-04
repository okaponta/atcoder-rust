use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!("{}", s.into_iter().filter(|c| c == &'1').count());
}
