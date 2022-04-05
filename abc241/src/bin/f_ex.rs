use std::collections::{BTreeSet, HashMap, VecDeque};

use maplit::{hashmap, hashset};

fn main() {
    proconio::input! {
        _: usize,
        _: usize,
        n: usize,
        s: (usize, usize),
        g: (usize, usize),
        xy: [(usize, usize); n],
    }

    let mut blocks_xy: HashMap<usize, BTreeSet<usize>> = hashmap! {};
    let mut blocks_yx: HashMap<usize, BTreeSet<usize>> = hashmap! {};

    for (x, y) in xy {
        blocks_xy.entry(x).or_default().insert(y);
        blocks_yx.entry(y).or_default().insert(x);
    }

    let mut visited = hashset! {};

    let mut qs = VecDeque::new();
    qs.push_back((s, 0usize));
    visited.insert(s);
    while let Some(((px, py), c)) = qs.pop_front() {
        let mut candidates = vec![];

        // y方向に滑る
        if let Some(bxy) = blocks_xy.get(&px) {
            if let Some(&by) = bxy.range(..py).next_back() {
                candidates.push((px, by + 1));
            }
            if let Some(&by) = bxy.range(py..).next() {
                candidates.push((px, by - 1));
            }
        }

        // x方向に滑る
        if let Some(byx) = blocks_yx.get(&py) {
            if let Some(&bx) = byx.range(..px).next_back() {
                candidates.push((bx + 1, py));
            }
            if let Some(&bx) = byx.range(px..).next() {
                candidates.push((bx - 1, py));
            }
        }

        let nc = c + 1;
        for np in candidates {
            if visited.insert(np) {
                if np == g {
                    println!("{}", nc);
                    return;
                }
                qs.push_back((np, nc));
            }
        }
    }

    println!("{}", -1);
}
