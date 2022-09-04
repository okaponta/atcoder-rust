use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 0.0;
    for _ in 0..n {
        let mut next = 0.0;
        for i in 1..7 {
            if ans < i as f64 {
                next += i as f64;
            } else {
                next += ans;
            }
        }
        ans = next / 6.0;
    }
    println!("{}", ans);
}
