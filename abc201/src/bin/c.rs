use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
    }
    let o = s.iter().filter(|&c| *c == 'o').count();
    let q = s.iter().filter(|&c| *c == '?').count();
    let ans = match o {
        0 => q * q * q * q,
        1 => (q + 1) * (q + 1) * (q + 1) * (q + 1) - q * q * q * q,
        2 => 2 * 2 * 2 * 2 - 2 + 4 * (2 * 2 * 2 - 2) * q + 6 * 2 * q * q,
        3 => 3 * (3 * 3 * 3 - 2 * 2 * 2 * 2 + 1) + 4 * 6 * q,
        4 => 4 * 3 * 2,
        _ => 0,
    };
    println!("{}", ans);
}
