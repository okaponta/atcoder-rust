use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        input! {
            n:usize,
            d:usize,
            k:Usize1,
        }
        let g = gcd(n, d);
        if g == 1 {
            println!("{}", k * d % n);
        } else {
            let cycle = n / g;
            let mut ans = k * d % n;
            ans += k / cycle;
            println!("{}", ans % n);
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
