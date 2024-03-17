use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut map = HashMap::new();
    let mut ans = 0;
    loop {
        while right < n && (map.len() < k || map.contains_key(&a[right])) {
            *map.entry(a[right]).or_insert(0) += 1;
            right += 1;
        }
        ans = ans.max(right - left);
        if right == n {
            break;
        }
        *map.entry(a[left]).or_insert(0) -= 1;
        if map[&a[left]] == 0 {
            map.remove(&a[left]);
        }
        left += 1;
    }
    println!("{}", ans);
}
