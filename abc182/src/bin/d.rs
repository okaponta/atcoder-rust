use std::vec;

use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[i64;n],
    }
    let mut sum = vec![0; n];
    let mut max = vec![0; n];
    let mut ini = vec![0; n];
    let mut ans = vec![0; n];
    sum[0] = a[0];
    max[0] = a[0];
    ans[0] = a[0];
    for i in 1..n {
        ini[i] = ini[i - 1] + sum[i - 1];
        sum[i] = sum[i - 1] + a[i];
        max[i] = max[i - 1].max(sum[i]);
        ans[i] = ini[i] + max[i];
    }
    println!("{}", ans.iter().max().unwrap().max(&0));
}
