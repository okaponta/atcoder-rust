use std::collections::{HashSet, VecDeque};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let mut ans = solve(&a);
    a.remove(0);
    ans = ans.max(solve(&a));
    println!("{}", ans);
}

fn solve(a: &Vec<usize>) -> usize {
    let n = a.len() / 2;
    let mut res = 0;
    let mut q = VecDeque::new();
    let mut set = HashSet::new();
    for i in 0..n {
        if a[2 * i] == a[2 * i + 1] {
            if !set.contains(&a[2 * i]) {
                set.insert(a[2 * i]);
                q.push_back(a[2 * i]);
            } else {
                res = res.max(set.len() * 2);
                while let Some(ai) = q.pop_front() {
                    if ai == a[2 * i] {
                        break;
                    }
                    set.remove(&ai);
                }
                q.push_back(a[2 * i]);
            }
        } else {
            res = res.max(set.len() * 2);
            set = HashSet::new();
            q = VecDeque::new();
        }
    }
    res.max(set.len() * 2)
}
