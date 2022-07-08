use proconio::input;

fn main() {
    input! {
        n:usize,
        mut v:[usize;n],
    }
    v.sort();
    let mut ans = v[0] as f64;
    for i in 1..n {
        ans = (ans + v[i] as f64) / 2.0;
    }
    println!("{}", ans);
}
