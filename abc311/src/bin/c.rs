use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;n],
    }
    let mut route = vec![0; n];
    let mut path = vec![];
    let mut tmp = 0;
    for i in 0..=n {
        if 0 < route[tmp] {
            path.drain(..route[tmp] - 1);
            println!("{}", path.len());
            println!("{}", path.iter().join(" "));
            return;
        }
        path.push(tmp + 1);
        route[tmp] = i + 1;
        tmp = a[tmp];
    }
}
