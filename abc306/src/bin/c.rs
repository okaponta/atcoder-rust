use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;3*n],
    }
    let mut count = vec![0; n];
    let mut ans = vec![];
    for i in 0..3 * n {
        count[a[i]] += 1;
        if count[a[i]] == 2 {
            ans.push(a[i] + 1);
        }
    }
    println!("{}", ans.iter().join(" "));
}
