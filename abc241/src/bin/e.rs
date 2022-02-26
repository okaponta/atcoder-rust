use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
        a:[usize;n],
    }

    let mut set = HashSet::new();
    let mut hist = vec![];
    let mut sum = vec![];
    let mut sum_last = 0;

    while !set.contains(&(sum_last % n)) {
        set.insert(sum_last % n);
        hist.push(sum_last % n);
        sum.push(sum_last);
        sum_last += a[sum_last % n];
    }

    if k < sum.len() {
        println!("{}", sum[k]);
        return;
    }

    let begin = hist
        .iter()
        .enumerate()
        .find(|&(_, x)| *x == sum_last % n)
        .unwrap()
        .0;

    let loop_size = sum_last - sum[begin];

    let first = sum[begin];
    let second = ((k - begin) / (sum.len() - begin)) * loop_size;
    let third = sum[((k - begin) % (sum.len() - begin)) + begin] - sum[begin];

    println!("{}", first + second + third);
}
