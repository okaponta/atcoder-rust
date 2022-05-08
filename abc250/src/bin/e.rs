use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
        b:[usize;n],
        q:usize,
        xy:[(Usize1,Usize1);q],
    }
    let mut a_idx = HashMap::new();
    for i in 0..n {
        if a_idx.contains_key(&a[i]) {
            continue;
        }
        a_idx.insert(a[i], i);
    }
    let mut b_idx = HashMap::new();
    for i in 0..n {
        if b_idx.contains_key(&b[i]) {
            continue;
        }
        b_idx.insert(b[i], i);
    }
    let mut a_must = vec![n; n];
    let mut a_max = 0;
    for i in 0..n {
        let num = a[i];
        if !b_idx.contains_key(&num) {
            // 以降不可能
            break;
        }
        a_max = a_max.max(*b_idx.get(&num).unwrap());
        a_must[i] = a_max;
    }
    let mut b_must = vec![n; n];
    let mut b_max = 0;
    for i in 0..n {
        let num = b[i];
        if !a_idx.contains_key(&num) {
            // 以降不可能
            break;
        }
        b_max = b_max.max(*a_idx.get(&num).unwrap());
        b_must[i] = b_max;
    }
    for (x, y) in xy {
        let b_req = a_must[x];
        let a_req = b_must[y];
        if a_req <= x && b_req <= y {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
