use std::vec;

use proconio::input;

fn main() {
    input! {
       n:usize,m:usize,
       a:[usize;n],
    }
    let mut map = vec![vec![-1]; n];
    for i in 0..n {
        map[a[i]].push(i as i32);
    }
    for i in 0..n {
        let target = &mut map[i];
        target.push(n as i32);
        for j in 0..target.len() - 1 {
            if target[j + 1] - target[j] > m as i32 {
                println!("{}", i);
                return;
            }
        }
    }
    println!("{}", n);
}
