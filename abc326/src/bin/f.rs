use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        x:i64,
        y:i64,
        a:[i64;n],
    }
    let mut xv = vec![];
    let mut yv = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            yv.push(a[i]);
        } else {
            xv.push(a[i]);
        }
    }
    let (x_ok, xb) = is_ok(xv, x);
    let (y_ok, yb) = is_ok(yv, y);
    if !x_ok || !y_ok {
        println!("No");
        return;
    }
    println!("Yes");
    let mut xd = true;
    let mut yd;
    let mut i = 0;
    let mut ans = vec![];
    loop {
        if yb.len() == i {
            break;
        }
        if (yb[i] && xd) || (!yb[i] && !xd) {
            ans.push('L');
        } else {
            ans.push('R');
        }
        yd = yb[i];
        if xb.len() == i {
            break;
        }
        if (!xb[i] && yd) || (xb[i] && !yd) {
            ans.push('L');
        } else {
            ans.push('R');
        }
        xd = xb[i];
        i += 1;
    }
    println!("{}", ans.iter().join(""));
}

fn is_ok(v: Vec<i64>, p: i64) -> (bool, Vec<bool>) {
    let n = v.len();
    let mut first = vec![];
    let mut second = vec![];
    for i in 0..n {
        if i < n / 2 {
            first.push(v[i]);
        } else {
            second.push(v[i]);
        }
    }
    let first_map = to_map(first);
    let second_map = to_map(second);
    for (k, mut v) in first_map {
        if second_map.contains_key(&(p - k)) {
            v.extend(second_map[&(p - k)].clone());
            return (true, v);
        }
    }
    (false, vec![])
}

fn to_map(v: Vec<i64>) -> HashMap<i64, Vec<bool>> {
    let n = v.len();
    let mut map = HashMap::new();
    if n == 0 {
        map.insert(0, vec![]);
        return map;
    }
    let mut q = VecDeque::new();
    q.push_back((0, vec![], 0));
    while let Some((k, mut v1, i)) = q.pop_front() {
        let mut v2 = v1.clone();
        v1.push(true);
        v2.push(false);
        if i == n - 1 {
            map.insert(k + v[i], v1);
            map.insert(k - v[i], v2);
        } else {
            q.push_back((k + v[i], v1, i + 1));
            q.push_back((k - v[i], v2, i + 1));
        }
    }
    map
}
