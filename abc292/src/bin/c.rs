use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut fac = vec![1; n + 1];
    for i in 2..=n {
        fac[i] = factorize(i as i64);
    }
    let mut ans = 0;
    for i in 1..n {
        ans += fac[i] * fac[n - i];
    }
    println!("{}", ans);
}

fn factorize(mut n: i64) -> usize {
    let mut res = 1;
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res *= ex + 1;
    }
    if n != 1 {
        res *= 2;
    }
    res
}
