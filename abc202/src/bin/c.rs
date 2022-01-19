use std::vec;

use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[usize;n],
       b:[usize;n],
       c:[usize;n],
    }
    let mut a_count = vec![0i64; 100001];
    for a in a {
        a_count[a] += 1;
    }
    let mut c_count = vec![0; 100001];
    for c in c {
        c_count[c] += 1;
    }
    let mut b_count = vec![0; 100001];
    for i in 0..n {
        b_count[b[i]] += c_count[i + 1];
    }
    let mut ans = 0;
    for i in 0..100001 {
        ans += a_count[i] * b_count[i];
    }
    println!("{}", ans);
}
