use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        x.push(xy[i].0);
        y.push(xy[i].1);
    }
    x.sort();
    y.sort();
    let midx = x[n / 2];
    let midy = y[n / 2];
    let mut ans = 0;
    for i in 0..n {
        ans += (midx - x[i]).abs();
        ans += (midy - y[i]).abs();
    }
    println!("{}", ans);
}
