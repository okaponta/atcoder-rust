use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;n],
    }
    let mut ans = 0i64;
    let mut count = 0;
    for i in 0..n {
        if i == a[i] {
            count += 1;
        }
        if i < a[i] {
            if a[a[i]] == i {
                ans += 1;
            }
        }
    }
    println!("{}", ans + count * (count - 1) / 2);
}
