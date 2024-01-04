use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        k:usize,
        ab:[(Usize1,usize);n],
    }
    let mut count = vec![0; 100_010];
    for (a, b) in ab {
        count[a] += b;
    }
    let s = ruiseki(&count);
    let ans = s.lower_bound(&k);
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
