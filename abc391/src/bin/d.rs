use proconio::{marker::*, *};
use std::collections::{HashSet, VecDeque};

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
    let mut list = vec![vec![(1 << 60, 0)]; w];
    for i in 0..n {
        list[xy[i].0].push((xy[i].1, i));
    }
    for i in 0..w {
        list[i].sort();
    }
    let mut cur = 0;
    let mut time = (0..w).into_iter().map(|i| list[i][cur].0).max().unwrap();
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
            for i in 0..w {
                set.insert(list[i][cur].1);
            }
            cur += 1;
            time = (0..w).into_iter().map(|i| list[i][cur].0).max().unwrap();
            qq.push_front((t, a, i));
        }
    }
    for i in 0..q {
        println!("{}", if ans[i] { "Yes" } else { "No" });
    }
}
