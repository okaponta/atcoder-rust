#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        _n:usize,
        _m:usize,
        q:usize,
    }
    let mut all = HashSet::new();
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {qi: u8, x: Usize1}
        if qi == 1 {
            input! {y:Usize1}
            map.entry(x).or_insert(HashSet::new()).insert(y);
        } else if qi == 2 {
            all.insert(x);
        } else {
            input! {y:Usize1}
            if all.contains(&x) {
                println!("Yes");
                continue;
            }
            if map.contains_key(&x) && map[&x].contains(&y) {
                println!("Yes");
                continue;
            }
            println!("No");
        }
    }
}
