use itertools::Itertools;
use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
       n:usize,
       xy:[(i64,i64);n],
    }
    let mut v = vec![];
    for p in xy.iter().permutations(2) {
        let x = p[0].0 - p[1].0;
        let y = p[0].1 - p[1].1;
        let gcd = gcd(x, y);
        v.push((x / gcd, y / gcd));
    }
    v.sort();
    v.dedup();
    println!("{}", v.len());
}
