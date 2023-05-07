use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut ans = n.saturating_sub(2) * m.saturating_sub(2);
    if n == 1 {
        ans = m.saturating_sub(2);
    }
    if m == 1 {
        ans = n.saturating_sub(2);
    }
    if n == 1 && m == 1 {
        ans = 1;
    }
    println!("{}", ans);
}
