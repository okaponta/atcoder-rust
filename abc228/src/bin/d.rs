use std::collections::BTreeSet;

use proconio::input;

const MOD: i64 = 1048576;

fn main() {
    input! {
       q:usize, tx:[(i64,i64);q],
    }
    let mut a: Vec<i64> = vec![-1; MOD as usize];
    let mut set = (0..=MOD).collect::<BTreeSet<_>>();
    for (t, x) in tx {
        if t == 1 {
            let mut h = x % MOD;
            h = *set.range(h..).next().unwrap();
            if h == MOD {
                h = *set.range(0..).next().unwrap();
            }
            set.remove(&h);
            a[h as usize] = x;
        } else {
            println!("{}", a[(x % MOD) as usize]);
        }
    }
}
