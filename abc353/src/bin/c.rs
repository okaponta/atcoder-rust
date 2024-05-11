use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    let modu = 100_000_000;
    let s = ruiseki(&a);
    let mut ans = 0;
    for i in 0..n {
        let ov = (n - a.lower_bound(&(modu - a[i]))).min(n - 1 - i);
        ans += s[n] - s[i + 1] + a[i] * (n - 1 - i);
        ans -= ov * modu;
    }
    println!("{}", ans);
}

// 累積和
fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
