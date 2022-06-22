use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,k:usize,
        s:Chars,
    }
    let mut prev = 'A';
    let mut count = 0;
    for c in s {
        if c == prev {
            count += 1;
        }
        prev = c;
    }
    let ans = (count + k * 2).min(n - 1);
    println!("{}", ans);
}
