use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        m:usize,
        l:usize,
    }
    let mut ans = 1 << 60;
    for i in 0..20 {
        for j in 0..15 {
            for k in 0..10 {
                if n <= 6 * i + 8 * j + 12 * k {
                    ans = ans.min(s * i + m * j + l * k);
                }
            }
        }
    }
    println!("{}", ans);
}
