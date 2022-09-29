use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
        bc:[(usize,usize);m],
    }
    a.sort();
    let mut heap = run_length_encode(a)
        .into_iter()
        .collect::<BinaryHeap<(usize, usize)>>();
    for (b, c) in bc {
        heap.push((c, b));
    }
    let mut ans = 0;
    let mut count = 0;
    while count < n {
        let (num, mut c) = heap.pop().unwrap();
        if n < count + c {
            c = n - count;
        }
        count += c;
        ans += num * c;
    }
    println!("{}", ans);
}

fn run_length_encode<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
