#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {n:usize, mut a:[i64;n]}
    if n <= 2 {
        println!("Yes");
        return;
    }
    let zero = a.iter().filter(|&i| &0 == i).count();
    if n - 1 <= zero {
        println!("Yes");
        return;
    }
    if 0 < zero {
        println!("No");
        return;
    }
    let count = a.iter().map(|&i| if 0 < i { 1 } else { -1 }).sum::<i64>();
    if count.abs() == n as i64 {
        a.sort();
        println!("{}", if judge(a) { "Yes" } else { "No" })
    } else if count.abs() == 1 {
        println!(
            "{}",
            if judge(rearrange(count, a)) {
                "Yes"
            } else {
                "No"
            }
        );
    } else if count.abs() == 0 {
        println!(
            "{}",
            if judge(rearrange(1, a.clone())) || judge(rearrange(-1, a)) {
                "Yes"
            } else {
                "No"
            }
        );
    } else {
        println!("No");
    }
}

fn rearrange(count: i64, a: Vec<i64>) -> Vec<i64> {
    let mut pos = vec![];
    let mut neg = vec![];
    for a in a {
        if 0 < a {
            pos.push(a);
        } else {
            neg.push(a);
        }
    }
    pos.sort();
    neg.sort();
    neg.reverse();
    let mut res = vec![];
    let mut is_pos = 0 <= count;
    while pos.len() != 0 || neg.len() != 0 {
        if is_pos {
            res.push(pos.pop().unwrap());
        } else {
            res.push(neg.pop().unwrap());
        }
        is_pos = !is_pos;
    }
    res
}

fn judge(a: Vec<i64>) -> bool {
    a.windows(3).all(|v| v[0] * v[2] == v[1] * v[1])
}
