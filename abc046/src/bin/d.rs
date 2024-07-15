use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let ans = s.iter().filter(|&&c| c == 'g').count() as i64 - (s.len() as i64 + 1) / 2;
    println!("{ans}");
}
