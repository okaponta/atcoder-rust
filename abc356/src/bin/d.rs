use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut count = vec![0; 60];
    let mut tmp = 1;
    for i in 0..60 {
        count[i] = f(n, tmp);
        tmp *= 2;
    }
    let mut ans = 0;
    let mo = 998244353;
    for i in 0..60 {
        if m >> i & 1 == 1 {
            ans += count[i] % mo;
            ans %= mo;
        }
    }
    println!("{}", ans);
}

fn f(n: usize, b: usize) -> usize {
    let mut res = 0;
    res += n / (2 * b) * b;
    let rem = n % (2 * b);
    if rem < b {
        return res;
    }
    res + rem + 1 - b
}
