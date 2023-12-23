use proconio::input;

fn main() {
    input! {
        a:i64,
        m:i64,
        mut l:i64,
        mut r:i64,
    }
    l -= a;
    r -= a;
    let mut k1 = l / m;
    let mut k2 = r / m;
    k1 -= 2;
    k2 += 2;
    while k1 * m < l {
        k1 += 1;
    }
    while r < k2 * m {
        k2 -= 1;
    }
    println!("{}", k2 - k1 + 1);
}
