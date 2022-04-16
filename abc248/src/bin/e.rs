use std::collections::HashMap;

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        xy:[(i64,i64);n],
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    // 組み合わせ -> index
    let mut map = HashMap::new();
    for (i, (one, two)) in (0..n).into_iter().tuple_combinations().enumerate() {
        map.insert((one, two), i);
    }
    let size = map.len();
    let mut uf = UnionFind::new(size);
    for (i, j, k) in (0..n).into_iter().tuple_combinations() {
        if is_on_line(xy[i].0, xy[i].1, xy[j].0, xy[j].1, xy[k].0, xy[k].1) {
            let one = map.get(&(i, j)).unwrap();
            let two = map.get(&(j, k)).unwrap();
            let three = map.get(&(i, k)).unwrap();
            uf.union(*one, *two);
            uf.union(*one, *three);
        }
    }
    let mut cmap = HashMap::new();
    for i in 2..301 {
        cmap.insert(i, i * (i - 1) / 2);
    }
    let target = cmap.get(&k).unwrap();
    let mut count = vec![0; size];
    for i in 0..size {
        count[uf.find(i)] += 1;
    }
    let ans = count.iter().filter(|e| e >= &target).count();
    println!("{}", ans);
}

fn is_on_line(x0: i64, y0: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    (y1 - y0) * (x2 - x0) == (y2 - y0) * (x1 - x0)
}
