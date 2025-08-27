#[rustfmt::skip]
use {ac_library::*,proconio::{marker::*, *},std::cmp::*,std::collections::*};

#[fastout]
fn main() {
    input! {mut n:usize, q:usize, mut xy:[(usize,usize);n]}
    let mut heap = BinaryHeap::new();
    let mut uf = Dsu::new(n + q);
    for i in 0..n {
        for j in i + 1..n {
            heap.push(Reverse((man(xy[i], xy[j]), i, j)));
        }
    }
    for _ in 0..q {
        input! {qi: u8}
        if qi == 1 {
            input! {a:usize, b:usize}
            for i in 0..n {
                heap.push(Reverse((man(xy[i], (a, b)), i, n)));
            }
            n += 1;
            xy.push((a, b));
        } else if qi == 2 {
            if uf.size(0) == n {
                println!("-1");
                continue;
            }
            let mut dist = 1 << 60;
            let mut target = vec![];
            while let Some(Reverse((d, i, j))) = heap.pop() {
                if uf.same(i, j) {
                    continue;
                }
                if dist != 1 << 60 && dist < d {
                    heap.push(Reverse((d, i, j)));
                    break;
                }
                dist = d;
                target.push((i, j));
            }
            println!("{dist}");
            for (i, j) in target {
                uf.merge(i, j);
            }
        } else {
            input! {u:Usize1, v:Usize1}
            println!("{}", if uf.same(u, v) { "Yes" } else { "No" });
        }
    }
}

fn man(p: (usize, usize), q: (usize, usize)) -> usize {
    p.0.abs_diff(q.0) + p.1.abs_diff(q.1)
}
