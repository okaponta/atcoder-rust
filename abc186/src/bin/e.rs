use proconio::input;

fn main() {
    input! {
       t:usize,
       nsk:[(i64,i64,i64);t]
    }
    for (n, s, k) in nsk {
        let g = gcd(n, gcd(s, k));

        if gcd(n / g, k / g) != 1 {
            println!("-1");
            continue;
        }
        let mut x = 0;
        let mut y = 0;
        let ng = n / g;
        extend_euclid(k / g, ng, &mut x, &mut y);
        println!("{}", ((x + ng) % ng) * (ng - s / g) % ng);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn extend_euclid(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let d = extend_euclid(b, a % b, y, x);
    *y -= a / b * *x;
    d
}
