use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        s:Chars,
    }
    let mut count = 0;
    let mut prev = 'A';
    for c in s {
        if c != prev {
            prev = c;
            count += 1;
        }
    }
    println!("{}", count);
}
