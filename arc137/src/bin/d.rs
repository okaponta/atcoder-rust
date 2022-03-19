use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        mut a:[usize;n],
    }
    let cap = n.max(m).next_power_of_two();
    a.reverse();
    a.resize(cap, 0);
    let mut bit = 0;
    while (1 << bit) < cap {
        for i in 0..cap {
            if (i & 1 << bit) == 0 {
                a[i | 1 << bit] ^= a[i];
            }
        }
        bit += 1;
    }
    a.reverse();
    a.truncate(m);
    println!(
        "{}",
        a.iter().map(|i| i.to_string()).collect_vec().join(" ")
    );
}
