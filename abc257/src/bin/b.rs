use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        q:usize,
        mut a:[usize;k],
        l:[Usize1;q],
    }
    for li in l {
        let cur = a[li];
        if li == k - 1 {
            if cur == n {
                continue;
            }
            a[li] += 1;
        } else {
            let next = a[li + 1];
            if cur + 1 == next {
                continue;
            }
            a[li] += 1;
        }
    }
    println!("{}", a.iter().join(" "));
}
