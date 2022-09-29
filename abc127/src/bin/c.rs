use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        lr:[(Usize1,Usize1);m],
    }
    let mut min = 0;
    let mut max = n;
    for (l, r) in lr {
        min = min.max(l);
        max = max.min(r);
    }
    if max < min {
        println!("0");
        return;
    }
    println!("{}", max + 1 - min);
}
