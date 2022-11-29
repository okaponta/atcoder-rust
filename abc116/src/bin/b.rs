use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s:usize,
    }
    let mut map = HashMap::new();
    let mut i = 1;
    let mut a = s;
    loop {
        if let Some(_) = map.get(&a) {
            println!("{}", i);
            return;
        }
        map.insert(a, i);
        a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
        i += 1;
    }
}
