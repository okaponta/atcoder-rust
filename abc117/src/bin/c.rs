use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut x:[i64;m],
    }
    x.sort();
    let mut diff = vec![];
    for i in 1..m {
        diff.push(x[i] - x[i - 1]);
    }
    diff.sort();
    let mut ans = 0;
    for i in 0..m.saturating_sub(n) {
        ans += diff[i];
    }
    println!("{}", ans);
}
