use proconio::input;

fn main() {
    input! {
        n:usize,
        s:i64,
        t:i64,
        mut w:i64,
        mut a:[i64;n-1],
    }
    a.insert(0, 0);
    let mut ans = 0;
    for a in a {
        w += a;
        if s <= w && w <= t {
            ans += 1;
        }
    }
    println!("{}", ans);
}
