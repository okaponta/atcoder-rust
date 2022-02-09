use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:usize,x:usize,m:usize
    }
    let mut set = HashSet::new();

    let mut path = vec![x];
    let mut sum = vec![x];

    let mut prev = x;
    set.insert(prev);
    let mut next = x * x % m;

    while !set.contains(&next) {
        set.insert(next);
        path.push(next);
        sum.push(sum.last().unwrap() + next);
        prev = next;
        next = (prev * prev) % m;
    }

    let mut begin = 0;
    for i in 0..path.len() {
        if path[i] == next {
            begin = i;
        }
    }

    let target = n - 1;
    let loop_size = path.len() - begin;
    let mut loop_sum = *sum.last().unwrap();
    if begin > 0 {
        loop_sum -= sum[begin - 1];
    }

    let ans = ((target - begin) / loop_size) * loop_sum + sum[begin + (target - begin) % loop_size];
    println!("{}", ans);
}
