use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        xy1:(i64,i64),
        xy2:(i64,i64),
    }
    let mut set = HashSet::new();
    set.insert(xy1);
    let mut next = HashSet::new();
    let mut ans = HashSet::new();
    for xy in set {
        next.insert((xy.0 + 1, xy.1 + 2));
        next.insert((xy.0 + 2, xy.1 + 1));
        next.insert((xy.0 - 1, xy.1 + 2));
        next.insert((xy.0 - 2, xy.1 + 1));
        next.insert((xy.0 + 1, xy.1 - 2));
        next.insert((xy.0 + 2, xy.1 - 1));
        next.insert((xy.0 - 1, xy.1 - 2));
        next.insert((xy.0 - 2, xy.1 - 1));
    }
    for xy in next {
        ans.insert((xy.0 + 1, xy.1 + 2));
        ans.insert((xy.0 + 2, xy.1 + 1));
        ans.insert((xy.0 - 1, xy.1 + 2));
        ans.insert((xy.0 - 2, xy.1 + 1));
        ans.insert((xy.0 + 1, xy.1 - 2));
        ans.insert((xy.0 + 2, xy.1 - 1));
        ans.insert((xy.0 - 1, xy.1 - 2));
        ans.insert((xy.0 - 2, xy.1 - 1));
    }
    println!("{}", if ans.contains(&xy2) { "Yes" } else { "No" });
}
