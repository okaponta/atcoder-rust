use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Excluded, Unbounded};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut min = BTreeMap::new();
    for i in 0..q {
        input! {c :u8}
        if c == 1 {
            input! {x :usize}
            let left = set.range(..=(x, i)).last();
            let right = set.range((x, i)..).next();
            update(&mut min, left, right, x, 1, -1);
            set.insert((x, i));
        } else if c == 2 {
            input! {x :usize}
            let b = set.range((x, 0)..).next().cloned().unwrap();
            let left = set.range(..b).last();
            let right = set.range((Excluded(b), Unbounded)).next();
            update(&mut min, left, right, x, -1, 1);
            set.remove(&b);
        } else {
            if let Some((a, _)) = min.iter().next() {
                println!("{}", a);
            }
        }
    }
}

fn update(
    min: &mut BTreeMap<usize, i32>,
    left: Option<&(usize, usize)>,
    right: Option<&(usize, usize)>,
    x: usize,
    edge: i32,
    center: i32,
) {
    if left.is_some() {
        update_key(min, left.unwrap().0 ^ x, edge);
    }
    if right.is_some() {
        update_key(min, right.unwrap().0 ^ x, edge);
    }
    if left.is_some() && right.is_some() {
        update_key(min, left.unwrap().0 ^ right.unwrap().0, center);
    }
}

fn update_key(min: &mut BTreeMap<usize, i32>, key: usize, add: i32) {
    *min.entry(key).or_insert(0) += add;
    if min[&key] == 0 {
        min.remove(&key);
    }
}
