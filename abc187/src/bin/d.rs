use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       mut ab:[(i64,i64);n],
    }
    let mut ao = 0;
    for (a, _) in &ab {
        ao += a;
    }
    let mut sabun = ab.iter().map(|(a, b)| 2 * a + b).collect_vec();
    sabun.sort();
    sabun.reverse();
    let mut tk = 0;
    let mut count = 0;
    for e in sabun {
        count += 1;
        tk += e;
        if tk > ao {
            println!("{}", count);
            return;
        }
    }
}
