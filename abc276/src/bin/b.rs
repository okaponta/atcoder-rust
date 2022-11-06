use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b + 1);
        edges[b].push(a + 1);
    }
    for i in 0..n {
        if edges[i].len() == 0 {
            println!("0");
            continue;
        }
        edges[i].sort();
        println!("{} {}", edges[i].len(), edges[i].iter().join(" "));
    }
}
