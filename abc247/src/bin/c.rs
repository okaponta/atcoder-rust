use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut prev = vec![];
    for i in 0..n {
        let mut next = prev.clone();
        next.push(i + 1);
        for e in prev {
            next.push(e);
        }
        prev = next;
    }
    println!("{}", prev.iter().join(" "));
}
