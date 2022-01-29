fn main() {
    proconio::input! {
        n: usize,
        c: i64,
        e: [(i64, i64, i64); n],
    }
    let mut x = vec![];
    for (a, b, c) in e {
        x.push((a, c));
        x.push((b + 1, -c));
    }
    x.sort();
    let mut sum = 0;
    let mut ans = 0;
    let mut pre = 0;
    for (t, v) in x {
        ans += (t - pre) * c.min(sum);
        sum += v;
        pre = t;
    }
    println!("{}", ans);
}
