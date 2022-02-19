use std::{collections::VecDeque, vec};

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        mut d:[i64;n],
        ab:[(Usize1,Usize1);m],
    }
    if d.iter().sum::<i64>() != (n as i64 - 1) * 2 {
        println!("-1");
        return;
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        d[a] -= 1;
        d[b] -= 1;
        uf.union(a, b);
    }

    let mut temp = vec![vec![]; n];

    for i in 0..n {
        if d[i] < 0 {
            println!("-1");
            return;
        }
        for _ in 0..d[i] {
            temp[uf.find(i)].push(i);
        }
    }

    let mut c1 = VecDeque::new();
    let mut c2 = vec![];

    for i in 0..n {
        if temp[i].len() == 1 {
            c1.push_back(temp[i][0]);
        } else if temp[i].len() > 1 {
            c2.push(vec![]);
            for lack in &temp[i] {
                let len = c2.len() - 1;
                c2[len].push(*lack);
            }
        }
    }

    let mut ans = vec![];

    for v in c2 {
        for i in 0..v.len() - 1 {
            if c1.is_empty() {
                println!("-1");
                return;
            }
            let back = c1.pop_back().unwrap();
            uf.union(v[i], back);
            ans.push((v[i], back));
        }
        c1.push_back(v[v.len() - 1]);
    }

    if c1.len() != 2 {
        println!("-1");
        return;
    }

    let first = c1.pop_front().unwrap();
    let second = c1.pop_front().unwrap();
    uf.union(first, second);
    ans.push((first, second));

    let rep = uf.find(0);
    for i in 0..n {
        if uf.find(i) != rep {
            println!("-1");
            return;
        }
    }

    for (from, to) in ans {
        println!("{} {}", from + 1, to + 1);
    }
}
