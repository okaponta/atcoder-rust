#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        st:[(String,usize);n],
    }
    let sum = st.iter().map(|(_, t)| t).sum::<usize>();
    for (s, t) in st {
        if sum < t * 2 {
            println!("{s}");
            return;
        }
    }
    println!("atcoder");
}
