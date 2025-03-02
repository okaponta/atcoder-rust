#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = vec![vec!['.'; n]; n];
    for i in 0..n {
        let j = n - i;
        if i <= j {
            for k in i..j {
                for l in i..j {
                    if i % 2 == 0 {
                        ans[k][l] = '#';
                    } else {
                        ans[k][l] = '.';
                    }
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", ans[i].iter().collect::<String>());
    }
}
