use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        mut a:[Usize1;n],
    }
    let mut count = 0;
    for i in 0..n - 1 {
        if a[i] == i {
            a.swap(i, i + 1);
            count += 1;
        }
    }
    if a[n - 1] == n - 1 {
        count += 1;
    }
    println!("{}", count);
}
