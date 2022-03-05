use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    s.sort();
    println!("{}", s.iter().collect::<String>());
}
