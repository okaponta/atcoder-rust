use proconio::input;
use superslice::Ext;

fn main() {
    input! {
       n:usize,
       t:usize,
       a:[usize;n],
    }
    let n1 = n / 2;
    let n2 = n - n1;
    // 前半部分の全列挙
    let mut first = vec![];
    for i in 0..1 << n1 {
        let mut sa = 0;
        for j in 0..n1 {
            if i >> j & 1 == 1 {
                sa += a[j];
            }
        }
        if sa <= t {
            first.push(sa);
        }
    }
    // 辞書順にソート
    first.sort();
    let mut ans = 0;
    // 後半部分の全列挙
    for i in 0..1 << n2 {
        let mut sa = 0;
        for j in 0..n2 {
            if i >> j & 1 == 1 {
                sa += a[n1 + j];
            }
        }
        if sa <= t {
            let pos = first.upper_bound(&(t - sa)) - 1;
            ans = ans.max(sa + first[pos]);
        }
    }
    println!("{}", ans);
}
