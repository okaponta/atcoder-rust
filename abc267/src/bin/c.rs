use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n]
    }
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + a[i]);
    }
    let mut ans = 0;
    for i in 0..m {
        ans += a[i] * (i as i64 + 1);
    }
    let mut tmp = ans;
    for i in 0..n - m {
        tmp -= s[m + i] - s[i];
        tmp += a[m + i] * m as i64;
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
