use std::collections::{HashSet, VecDeque};

use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        q:usize,
        abc:[(Usize1,Usize1,i64);m],
        xy:[(Usize1,Usize1);q],
    }
    let mut edges = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    let mut self_loop = vec![];
    for (a, b, c) in abc {
        if a == b && c != 0 {
            self_loop.push(a);
            continue;
        }
        uf.union(a, b);
        edges[a].push((b, c));
        edges[b].push((a, -c));
    }

    let mut infset = HashSet::new();
    for e in self_loop {
        infset.insert(uf.find(e));
    }

    let mut visited = vec![false; n];
    let mut pot = vec![0; n];
    let mut q = VecDeque::new();
    let rep = (0..n)
        .into_iter()
        .map(|i| uf.find(i))
        .collect::<HashSet<usize>>();
    for e in rep {
        if infset.contains(&e) {
            continue;
        }
        q.push_back((e, 0));
    }
    while let Some((e, h)) = q.pop_front() {
        if infset.contains(&e) {
            continue;
        }
        if visited[e] {
            if pot[e] != h {
                infset.insert(uf.find(e));
            }
            continue;
        }
        visited[e] = true;
        pot[e] = h;
        for &(next, cost) in &edges[e] {
            if visited[next] {
                if pot[next] != h + cost {
                    infset.insert(uf.find(e));
                }
                continue;
            }
            q.push_back((next, h + cost));
        }
    }
    for (x, y) in xy {
        if !uf.equiv(x, y) {
            println!("nan");
            continue;
        }
        if infset.contains(&uf.find(x)) {
            println!("inf");
            continue;
        }
        println!("{}", pot[y] - pot[x]);
    }
}
