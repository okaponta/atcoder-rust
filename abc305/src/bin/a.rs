use proconio::input;

fn main() {
    input! {
        n:i32,
    }
    let mut min = 100;
    let mut ans = 0;
    for i in (0..=100).step_by(5) {
        if (n - i).abs() < min {
            min = (n - i).abs();
            ans = i;
        }
    }
    println!("{}", ans);
}
