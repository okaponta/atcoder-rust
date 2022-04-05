use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        _:usize,_:usize,n:usize,
        s:(Usize1,Usize1),
        g:(Usize1,Usize1),
        xy:[(Usize1,Usize1);n],
    }
    let x_set: HashSet<usize> = xy.iter().map(|p| p.0).collect();
    let y_set: HashSet<usize> = xy.iter().map(|p| p.1).collect();
    let mut x_sorted = xy.clone();
    x_sorted.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut y_sorted = xy.iter().map(|&(x, y)| (y, x)).collect_vec();
    y_sorted.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((s.0, s.1, 0));
    while let Some(p) = q.pop_front() {
        if p.0 == g.0 && p.1 == g.1 {
            println!("{}", p.2);
            return;
        }
        if visited.contains(&(p.0, p.1)) {
            continue;
        }
        visited.insert((p.0, p.1));
        //xに存在するか
        if x_set.contains(&p.0) {
            let index = x_sorted.lower_bound(&(p.0, p.1));
            if index == 0 {
                let n1 = x_sorted[index];
                q.push_back((n1.0, n1.1 - 1, p.2 + 1));
            } else if index == n {
                let n1 = x_sorted[index - 1];
                q.push_back((n1.0, n1.1 + 1, p.2 + 1));
            } else {
                let n1 = x_sorted[index - 1];
                let n2 = x_sorted[index];
                if n1.0 == p.0 && n2.0 == p.0 {
                    q.push_back((n1.0, n1.1 + 1, p.2 + 1));
                    q.push_back((n2.0, n2.1 - 1, p.2 + 1));
                } else if n1.0 == p.0 {
                    if n1.1 > p.1 {
                        q.push_back((n1.0, n1.1 - 1, p.2 + 1));
                    } else {
                        q.push_back((n1.0, n1.1 + 1, p.2 + 1));
                    }
                } else {
                    if n2.1 > p.1 {
                        q.push_back((n2.0, n2.1 - 1, p.2 + 1));
                    } else {
                        q.push_back((n2.0, n2.1 + 1, p.2 + 1));
                    }
                }
            }
        }
        //yに存在するか
        if y_set.contains(&p.1) {
            let index = y_sorted.lower_bound(&(p.1, p.0));
            if index == 0 {
                let n1 = y_sorted[index];
                let n1 = (n1.1, n1.0);
                q.push_back((n1.0 - 1, n1.1, p.2 + 1));
            } else if index == n {
                let n1 = y_sorted[index - 1];
                let n1 = (n1.1, n1.0);
                q.push_back((n1.0 + 1, n1.1, p.2 + 1));
            } else {
                let n1 = y_sorted[index - 1];
                let n1 = (n1.1, n1.0);
                let n2 = y_sorted[index];
                let n2 = (n2.1, n2.0);
                if n1.1 == p.1 && n2.1 == p.1 {
                    q.push_back((n1.0 + 1, n1.1, p.2 + 1));
                    q.push_back((n2.0 - 1, n2.1, p.2 + 1));
                } else if n1.1 == p.1 {
                    if n1.0 > p.0 {
                        q.push_back((n1.0 - 1, n1.1, p.2 + 1));
                    } else {
                        q.push_back((n1.0 + 1, n1.1, p.2 + 1));
                    }
                } else {
                    if n2.0 > p.0 {
                        q.push_back((n2.0 - 1, n2.1, p.2 + 1));
                    } else {
                        q.push_back((n2.0 + 1, n2.1, p.2 + 1));
                    }
                }
            }
        }
    }
    println!("{}", -1);
}
