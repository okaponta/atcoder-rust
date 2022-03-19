use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        a:[usize;n],
    }
    let mut ans = vec![0; m];
    for i in 0..m {
        let mut ansi = 0;
        for j in 0..n {
            if is_nck_odd(i + n - j - 1, n - j - 1) {
                ansi = ansi ^ a[j];
            }
        }
        ans[i] = ansi;
    }
    println!(
        "{}",
        ans.iter().map(|i| i.to_string()).collect_vec().join(" ")
    );
}

fn is_nck_odd(n: usize, k: usize) -> bool {
    n & k == k
}
