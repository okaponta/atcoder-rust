use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;n],
        b:[usize;n],
        c:[usize;n-1],
    }
    let mut ans = b[a[0]];
    for i in 1..n {
        ans += b[a[i]];
        if a[i - 1] + 1 == a[i] {
            ans += c[a[i - 1]];
        }
    }
    println!("{}", ans);
}
