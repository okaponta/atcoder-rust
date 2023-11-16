use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let s = a.iter().sum::<i64>();
    let mut tmp = 0;
    let mut ans = 1 << 60;
    for i in 0..n - 1 {
        tmp += a[i];
        ans = ans.min((2 * tmp - s).abs());
    }
    println!("{}", ans);
}
