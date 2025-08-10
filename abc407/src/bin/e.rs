use {proconio::*, std::collections::*};

fn main() {
    input! {t:usize}
    (0..t).into_iter().for_each(|_| case());
}

fn case() {
    input! {n:usize, a:[usize;2*n]}
    let mut ans = a[0];
    let mut heap = BinaryHeap::new();
    for i in 1..n {
        heap.push(a[2 * i]);
        heap.push(a[2 * i - 1]);
        ans += heap.pop().unwrap();
    }
    println!("{}", ans);
}
