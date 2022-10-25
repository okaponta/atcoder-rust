use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[Usize1;n],
    }
    let mut ans = vec![0; 2 * n + 1];
    for i in 0..n {
        ans[2 * i + 1] = ans[a[i]] + 1;
        ans[2 * i + 2] = ans[a[i]] + 1;
    }
    for a in ans {
        println!("{}", a);
    }
}
