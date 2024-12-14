#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut s:i64,
        mut a:[i64;n],
    }
    let sum = a.iter().sum::<i64>();
    s %= sum;
    for i in 0..n {
        a.push(a[i]);
    }
    let mut tmp = 0;
    let mut right = 0;
    for left in 0..n {
        while tmp + a[right] <= s {
            tmp += a[right];
            right += 1;
        }
        if tmp == s {
            println!("Yes");
            return;
        }
        tmp -= a[left];
    }
    println!("No");
}
