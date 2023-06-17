use proconio::input;

fn main() {
    input! {
        a:[u128;64],
    }
    let mut ans = 0;
    for i in 0..64 {
        ans += a[i] << i;
    }
    println!("{}", ans);
}
