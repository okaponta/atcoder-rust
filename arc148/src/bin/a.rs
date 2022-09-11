use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    a.sort();
    a.dedup();
    let n = a.len();
    if n == 1 {
        println!("1");
        return;
    }
    if n == 2 {
        if a[1] - a[0] == 1 {
            println!("2");
        } else {
            println!("1");
        }
        return;
    }
    let mut sub = vec![];
    for i in 0..n - 1 {
        sub.push(a[i + 1] - a[i]);
    }
    let mut g = gcd(sub[0], sub[1]);
    for i in sub {
        g = gcd(g, i);
    }
    println!("{}", if 1 < g { 1 } else { 2 });
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
