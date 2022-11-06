use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut target = 0;
    for i in 0..n {
        target = gcd(a[i], target);
    }
    let mut ans = 0;
    for i in 0..n {
        let div = count(a[i], target);
        if div == -1 {
            println!("-1");
            return;
        } else {
            ans += div;
        }
    }
    println!("{}", ans);
}

fn count(num: usize, gcd: usize) -> i64 {
    let mut div = num / gcd;
    let mut res = 0;
    while div % 2 == 0 {
        div /= 2;
        res += 1;
    }
    while div % 3 == 0 {
        div /= 3;
        res += 1;
    }
    if div != 1 {
        -1
    } else {
        res
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
