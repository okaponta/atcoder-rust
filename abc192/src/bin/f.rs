use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
       n:usize,
       x:usize,
       a:[usize;n],
    }
    let mut ans = 1 << 60;
    for i in 1..=n {
        let mut rem = vec![vec![0; i]; i + 1];
        for j in 0..n {
            for (k, l) in iproduct!((0..i).rev(), 0..i) {
                if rem[k][l] != 0 {
                    rem[k + 1][(l + a[j]) % i] = rem[k + 1][(l + a[j]) % i].max(rem[k][l] + a[j]);
                }
            }
            rem[1][a[j] % i] = a[j];
        }
        if rem[i][x % i] != 0 {
            ans = ans.min((x - rem[i][x % i]) / i);
        }
    }
    println!("{}", ans);
}
