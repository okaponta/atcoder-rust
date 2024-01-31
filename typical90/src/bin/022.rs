use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    let g = gcd(a, gcd(b, c));
    let ans = a / g + b / g + c / g;
    println!("{}", ans - 3);
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
