use proconio::input;

fn main() {
    input! {
        a:usize,
        x:usize,
        m:usize,
    }
    let mut pow = vec![1, a % m];
    let mut s = vec![1, (a + 1) % m];
    for i in 2..45 {
        pow.push((pow[i - 1] * pow[i - 1]) % m);
        s.push((s[i - 1] + s[i - 1] * pow[i]) % m);
    }
    let mut ans = 0;
    let mut cur = 1;
    for i in (0..45).rev() {
        if x >> i & 1 == 1 {
            ans += s[i] * cur;
            cur *= pow[i + 1];
            ans %= m;
            cur %= m;
        }
    }
    println!("{}", ans);
}
