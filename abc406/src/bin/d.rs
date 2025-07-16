#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        xy:[(Usize1,Usize1);n],
        q:usize,
        qx:[(u8,Usize1);q],
    }
    let mut row = vec![HashSet::new(); h];
    let mut col = vec![HashSet::new(); w];
    for (x, y) in xy {
        row[x].insert(y);
        col[y].insert(x);
    }
    for (q, x) in qx {
        if q == 1 {
            println!("{}", row[x].len());
            row[x].iter().for_each(|i| {
                col[*i].remove(&x);
            });
            row[x].clear();
        } else {
            println!("{}", col[x].len());
            col[x].iter().for_each(|i| {
                row[*i].remove(&x);
            });
            col[x].clear();
        }
    }
}
