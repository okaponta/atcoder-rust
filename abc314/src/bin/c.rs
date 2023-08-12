use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:Chars,
        c:[Usize1;n],
    }
    let mut t = s.clone();
    let mut colors = vec![vec![]; m];
    for i in 0..n {
        colors[c[i]].push(i);
    }
    let mut index = vec![0; n];
    for i in 0..m {
        for j in 1..=colors[i].len() {
            index[colors[i][j % colors[i].len()]] = colors[i][j - 1];
        }
    }
    for i in 0..n {
        t[i] = s[index[i]];
    }
    println!("{}", t.iter().join(""));
}
