use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|i| a[*i]);
    order.reverse();
    let mut flag = vec![1; n];
    let mut sum = 0;
    for i in 0..(n / 2) {
        sum += a[order[i]];
        flag[order[i]] = -1;
    }
    let s = flag
        .iter()
        .scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .collect::<Vec<_>>();
    let k = s.iter().position_min().unwrap();
    println!("{} {}", (k + 1) % n, sum);
}
