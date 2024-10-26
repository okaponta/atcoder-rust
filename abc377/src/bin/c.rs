use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:i64,
        m:usize,
        ab:[(i64,i64);m],
    }
    let mut set = HashSet::new();
    let dx = vec![0, 1, 2, 2, 1, -1, -2, -2, -1];
    let dy = vec![0, 2, 1, -1, -2, -2, -1, 1, 2];
    for (a, b) in ab {
        for i in 0..9 {
            let nx = a + dx[i];
            let ny = b + dy[i];
            if nx < 1 || n < nx || ny < 1 || n < ny {
                continue;
            }
            set.insert((nx, ny));
        }
    }
    println!("{}", n * n - set.len() as i64);
}
