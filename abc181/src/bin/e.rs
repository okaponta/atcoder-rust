use proconio::input;
use superslice::Ext;

fn main() {
    input! {
       n:usize,m:usize,
       mut h:[i64;n],
       w:[i64;m],
    }
    h.sort();
    let mut l = vec![0];
    let mut r = vec![0];
    for i in 1..n {
        let diff = h[i] - h[i - 1];
        if i % 2 != 0 {
            l.push(diff);
        } else {
            r.push(diff);
        }
    }
    let a = l
        .iter()
        .scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .collect::<Vec<_>>();
    let b = r
        .iter()
        .scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .collect::<Vec<_>>();
    let mut ans = 1 << 60;
    for wi in w {
        let i = h.lower_bound(&wi);
        let pair = if i % 2 != 0 { i - 1 } else { i };
        ans = ans.min((h[pair] - wi).abs() + a[i / 2] + b[n / 2] - b[i / 2]);
    }
    println!("{}", ans);
}
