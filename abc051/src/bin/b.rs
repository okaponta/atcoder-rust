use proconio::input;

fn main() {
    input! {
        k:usize,
        s:usize,
    }
    let mut ans = 0;
    for x in 0..=k {
        for y in 0..=k {
            if x + y <= s && s - x - y <= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
