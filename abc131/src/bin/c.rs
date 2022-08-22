use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a:Usize1,
        b:usize,
        c:usize,
        d:usize,
    }
    let divc = divnum(a, b, c);
    let divd = divnum(a, b, d);
    let divcd = divnum(a, b, lcm(c, d));
    println!("{}", b - a + divcd - divc - divd);
}

fn divnum(a: usize, b: usize, c: usize) -> usize {
    b / c - a / c
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
