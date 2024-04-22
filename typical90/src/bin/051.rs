use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        k:usize,
        p:usize,
        a:[usize;n],
    }
    let m = n / 2;
    let mut one = vec![vec![]; k + 1];
    for i in 0usize..1 << m {
        let mut tmp = 0;
        let mut cnt = 0;
        for j in 0..m {
            if i >> j & 1 == 1 {
                tmp += a[j];
                cnt += 1;
            }
        }
        if k < cnt || p < tmp {
            continue;
        }
        one[cnt].push((tmp, i));
    }
    for i in 0..=k {
        one[i].sort();
    }
    let mut ans = 0;
    for i in 0usize..1 << (n - m) {
        let mut tmp = 0;
        let mut cnt = 0;
        for j in 0..(n - m) {
            if i >> j & 1 == 1 {
                tmp += a[m + j];
                cnt += 1;
            }
        }
        if k < cnt || p < tmp {
            continue;
        }
        ans += one[k - cnt].lower_bound(&(p - tmp, 1 << 60));
    }
    println!("{}", ans);
}
