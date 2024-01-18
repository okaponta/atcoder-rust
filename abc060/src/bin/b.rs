use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    let g = gcd(a, b);
    println!("{}", if c % g == 0 { "YES" } else { "NO" });
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
