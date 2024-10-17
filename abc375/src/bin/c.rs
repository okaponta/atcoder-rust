#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[Chars;n],
    }
    let rot0 = rot(n, &a);
    let rot1 = rot(n, &rot0);
    let rot2 = rot(n, &rot1);
    let rot3 = rot(n, &rot2);
    let rot = vec![rot0, rot1, rot2, rot3];
    let mut ans = vec![vec!['.'; n]; n];
    for i in 0..n {
        let o1 = i.min(n - 1 - i);
        for j in 0..n {
            let o2 = j.min(n - 1 - j);
            ans[i][j] = rot[o1.min(o2) % 4][i][j];
        }
    }
    for i in 0..n {
        println!("{}", ans[i].iter().join(""));
    }
}

fn rot(n: usize, a: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            res[j][n - 1 - i] = a[i][j]
        }
    }
    res
}
