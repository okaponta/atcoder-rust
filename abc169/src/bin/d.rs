use num_integer::Roots;
use proconio::input;

fn factorize(mut n: i64) -> Vec<(i64, i32)> {
    let mut res = vec![];
    for a in 2..=n.sqrt() {
        if n % a != 0 {
            continue;
        }
        let mut ex = 0;
        while n % a == 0 {
            ex += 1;
            n /= a;
        }
        res.push((a, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

fn main() {
    input! {
       n:i64,
    }
    let factor = factorize(n);
    let mut ans = 0;
    for mut e in factor {
        let mut count = 0;
        while e.1 > count {
            ans += 1;
            count += 1;
            e.1 -= count;
        }
    }
    println!("{}", ans);
}
