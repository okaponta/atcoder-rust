use proconio::input;

fn main() {
    input! {
        x:usize,
    }
    let mut ans = (x / 11) * 2;
    if x % 11 != 0 {
        ans += 1;
        if 6 < x % 11 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
