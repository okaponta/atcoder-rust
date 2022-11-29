use proconio::input;

fn main() {
    input! {
        n:usize,
        mut h:[usize;n],
    }
    let mut ans = 0;
    for i in 0..n {
        while h[i] != 0 {
            ans += 1;
            let mut j = i;
            while j < n && h[j] != 0 {
                h[j] -= 1;
                j += 1;
            }
        }
    }
    println!("{}", ans);
}
