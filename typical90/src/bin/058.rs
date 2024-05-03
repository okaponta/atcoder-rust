use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    println!("{}", index_with_loop(n, k));
}

fn index_with_loop(init: usize, k: usize) -> usize {
    fn next(mut n: usize) -> usize {
        let mut res = n;
        for i in vec![10000, 1000, 100, 10, 1] {
            res += n / i;
            n %= i;
        }
        res % 100000
    }

    let mut v = vec![];
    let mut set = HashSet::new();
    let mut tmp = init;
    while !set.contains(&tmp) {
        v.push(tmp);
        set.insert(tmp);
        tmp = next(tmp);
    }
    if k < v.len() {
        return v[k];
    }
    let offset = v.iter().position(|&x| x == tmp).unwrap();
    let l = v.len() - offset;
    v[offset + (k - offset) % l]
}
