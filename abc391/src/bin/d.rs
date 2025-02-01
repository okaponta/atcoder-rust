use std::collections::{HashSet, VecDeque};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        w:usize,
        xy:[(Usize1,Usize1);n],
        q:usize,
        ta:[(usize,Usize1);q],
    }
    let mut tai = vec![];
    for i in 0..q {
        tai.push((ta[i].0, ta[i].1, i));
    }
    tai.sort();
    let mut list = vec![vec![(1 << 60, 0); 2]; w];
    for i in 0..n {
        list[xy[i].0].push((xy[i].1, i));
    }
    for i in 0..w {
        list[i].sort();
        list[i].reverse();
    }
    let mut tmp = vec![(0, 0); w];
    for i in 0..w {
        tmp[i] = list[i].pop().unwrap();
    }
    let mut tmpmax = tmp.iter().max().unwrap().0;
    let mut time = tmpmax;
    let mut set = HashSet::new();
    let mut ans = vec![false; q];
    let mut qq = VecDeque::new();
    for (t, a, i) in tai {
        qq.push_back((t, a, i));
    }
    while let Some((t, a, i)) = qq.pop_front() {
        if t <= time {
            ans[i] = !set.contains(&a);
        } else {
            for (_, i) in tmp {
                set.insert(i);
            }
            tmp = vec![(0, 0); w];
            for i in 0..w {
                tmp[i] = list[i].pop().unwrap();
            }
            tmpmax = tmp.iter().max().unwrap().0;
            time = tmpmax;
            qq.push_front((t, a, i));
        }
    }
    for i in 0..q {
        println!("{}", if ans[i] { "Yes" } else { "No" });
    }
}
