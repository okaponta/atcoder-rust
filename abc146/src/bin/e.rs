use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    if k == 1 {
        println!("0");
        return;
    }
    let mut map = HashMap::from([(0, 1usize)]);
    let mut s = 0;
    let mut ans = 0;
    let mut rem = HashMap::from([(k - 2, 0)]);
    for i in 0..n {
        s += a[i];
        let key = (k + s - (i + 1)) % k;
        ans += map.get(&key).unwrap_or(&0);
        *map.entry(key).or_insert(0) += 1;
        rem.insert(i + k - 1, key);
        if rem.contains_key(&i) {
            *map.entry(rem[&i]).or_insert(0) -= 1;
        }
    }
    println!("{}", ans);
}
