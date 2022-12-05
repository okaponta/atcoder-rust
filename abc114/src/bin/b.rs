use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars
    }
    println!(
        "{}",
        s.windows(3)
            .map(|v| diff(dig(v[0]) * 100 + dig(v[1]) * 10 + dig(v[2]), 753))
            .min()
            .unwrap()
    );
}

fn dig(c: char) -> usize {
    c.to_digit(10).unwrap() as usize
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
