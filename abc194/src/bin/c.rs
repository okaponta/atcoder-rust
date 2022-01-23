use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut map = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }
    let mut ans = 0;
    for ai in &map {
        for aj in &map {
            if ai.0 <= aj.0 {
                continue;
            }
            ans += (ai.0 - aj.0) * (ai.0 - aj.0) * ai.1 * aj.1;
        }
    }
    println!("{}", ans);
}
