#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        abw:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![vec![]; 1024]; n];
    for (a, b, w) in abw {
        for i in 0..1024 {
            g[a][i].push((b, i ^ w));
        }
    }
    let mut q = VecDeque::new();
    let mut set = HashSet::new();
    q.push_back((0, 0));
    while let Some(cur) = q.pop_front() {
        for &next in &g[cur.0][cur.1] {
            if set.contains(&next) {
                continue;
            }
            set.insert(next);
            q.push_back(next);
        }
    }
    let mut res = 1 << 20;
    for i in 0..1024 {
        if set.contains(&(n - 1, i)) {
            res = res.min(i);
        }
    }
    if res == 1 << 20 {
        println!("-1");
    } else {
        println!("{}", res)
    }
}
