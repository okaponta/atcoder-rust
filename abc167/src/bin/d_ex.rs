use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [Usize1; n],
    }
    let mut ans = 0;
    while k > 0 {
        if k & 1 > 0 {
            ans = a[ans];
        }
        a = a.iter().map(|&i| a[i]).collect();
        k >>= 1;
    }
    println!("{}", ans + 1);
}
