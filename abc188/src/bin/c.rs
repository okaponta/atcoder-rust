use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       mut n:usize,
       a:[i32;1<<n],
    }
    let mut winner = (0..1 << n).collect_vec();
    while n != 1 {
        n -= 1;
        let mut next = vec![0; 1 << n];
        for i in 0..1 << n {
            let pl1 = winner[i * 2];
            let pl2 = winner[i * 2 + 1];
            if a[pl1] > a[pl2] {
                next[i] = pl1;
            } else {
                next[i] = pl2;
            }
        }
        winner = next;
    }
    println!(
        "{}",
        if a[winner[0]] > a[winner[1]] {
            winner[1] + 1
        } else {
            winner[0] + 1
        }
    );
}
