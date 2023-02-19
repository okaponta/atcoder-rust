use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        b:[Usize1;m],
    }
    let mut ans = 0;
    for i in 0..m {
        ans += a[b[i]];
    }
    println!("{}", ans);
}
