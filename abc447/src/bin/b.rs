#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
    }
    let mut map = HashMap::new();
    s.iter().for_each(|&c| *map.entry(c).or_insert(0) += 1);
    let max = *map.values().max().unwrap();
    let set = map
        .into_iter()
        .filter(|e| e.1 == max)
        .map(|e| e.0)
        .collect::<HashSet<_>>();
    println!(
        "{}",
        s.into_iter()
            .filter(|c| !set.contains(c))
            .collect::<String>()
    );
}
