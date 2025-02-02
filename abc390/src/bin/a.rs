#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut a:[usize;5],
    }
    let ans = vec![1, 2, 3, 4, 5];
    if a == ans {
        println!("No");
        return;
    }
    for i in 0..4 {
        if a[i] != i + 1 {
            a.swap(i, i + 1);
            break;
        }
    }
    if a == ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
