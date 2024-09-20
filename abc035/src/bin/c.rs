use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,
       q:usize,
       lr:[(Usize1,usize);q],
    }
    let mut flip = vec![false; n + 1];
    for (l, r) in lr {
        flip[l] = !flip[l];
        flip[r] = !flip[r];
    }
    let mut ans = vec![if flip[0] { 1u8 } else { 0 }];
    for i in 1..n {
        ans.push(if flip[i] { 1 - ans[i - 1] } else { ans[i - 1] });
    }
    println!("{}", ans.iter().join(""));
}
