use proconio::input;

fn main() {
    input! {
        l:i64,r:i64,
    }
    let mut count = 0;
    loop {
        for i in 0..=count {
            if gcd(l + i, r - count + i) == 1 {
                println!("{}", r - l - count);
                return;
            }
        }
        count += 1;
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
