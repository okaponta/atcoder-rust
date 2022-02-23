use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map = std::collections::BTreeMap::new();
    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }
    let max = *map.values().max().unwrap();
    for (s, c) in map {
        if c == max {
            println!("{}", s);
        }
    }
}
