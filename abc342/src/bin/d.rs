use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    let mut count = vec![0; 200_000];
    let mut zero = n - 1;
    for a in a {
        if a == 0 {
            ans += zero;
            zero -= 1;
            continue;
        }
        let tmp = calc(a);
        ans += count[tmp];
        count[tmp] += 1;
    }
    println!("{}", ans);
}

fn calc(a: usize) -> usize {
    let mut res = 1;
    for (a, b) in factorize(a) {
        if b % 2 == 1 {
            res *= a;
        }
    }
    res
}

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
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
