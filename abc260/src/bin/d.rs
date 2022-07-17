use std::collections::{BTreeSet, HashMap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,k:usize,
        p:[i64;n],
    }
    if k == 1 {
        let mut ans = vec![0; n];
        for i in 0..n {
            ans[p[i] as usize - 1] = i + 1;
        }
        for pi in ans {
            println!("{}", pi);
        }
        return;
    }
    let mut ans = vec![-1; n];
    let mut set = BTreeSet::new();
    let mut map: HashMap<i64, Vec<i64>> = HashMap::new();
    for i in 0..n {
        if let Some(&(x, num)) = set.range((p[i], 0)..).next() {
            if num == k - 1 {
                // 食べる
                set.remove(&(x, num));
                ans[p[i] as usize - 1] = i as i64 + 1;
                let v = map.remove(&x).unwrap_or(Vec::new());
                for vi in v {
                    ans[vi as usize - 1] = i as i64 + 1;
                }
            } else {
                //重ねる
                set.remove(&(x, num));
                set.insert((p[i], num + 1));
                let mut val = map.remove(&x).unwrap_or(Vec::new());
                val.push(p[i]);
                map.insert(p[i], val);
            }
        } else {
            set.insert((p[i], 1));
            map.insert(p[i], vec![p[i]]);
        }
    }
    for pi in ans {
        println!("{}", pi);
    }
}
