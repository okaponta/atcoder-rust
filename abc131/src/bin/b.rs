use proconio::input;

fn main() {
    input! {
        n:i64,
        l:i64
    }
    let mut a = vec![];
    let mut ans = 0;
    for i in 0..n {
        ans += l + i;
        a.push(l + i);
    }
    a.sort_by(|a, b| a.abs().cmp(&b.abs()));
    println!("{}", ans - a[0]);
}
