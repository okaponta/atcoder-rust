use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut map = HashMap::new();
    for si in s {
        if let Some(num) = map.get(&si) {
            println!("{}({})", si, num);
        } else {
            println!("{}", si);
        }
        *map.entry(si).or_insert(0) += 1;
    }
}
