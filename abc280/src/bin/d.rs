use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        k:i64,
    }
    let factor = factorize(k);
    let mut ans = 0;
    for (p, x) in factor {
        let mut temp = 0;
        let mut count = 0;
        while count < x {
            temp += p;
            count += calcpow(p, temp);
        }
        ans = ans.max(temp);
    }
    println!("{}", ans);
}

fn calcpow(p: i64, mut temp: i64) -> i64 {
    let mut res = 0;
    while temp % p == 0 {
        temp /= p;
        res += 1;
    }
    res
}

fn factorize(mut n: i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
