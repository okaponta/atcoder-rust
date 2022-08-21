use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut t:i64,
        a:[i64;n-1],
        xy:[(Usize1,i64);m],
    }
    let mut bonus = vec![0; n];
    for (x, y) in xy {
        bonus[x] += y;
    }
    t += bonus[0];
    for i in 0..n - 1 {
        if t - a[i] <= 0 {
            println!("No");
            return;
        }
        t -= a[i];
        t += bonus[i + 1];
    }
    println!("Yes");
}
