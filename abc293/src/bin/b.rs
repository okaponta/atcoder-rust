use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;n],
    }
    let mut used = vec![false; n];
    for i in 0..n {
        if used[i] {
            continue;
        }
        used[a[i]] = true;
    }
    let mut ans = vec![];
    for i in 0..n {
        if !used[i] {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
