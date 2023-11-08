use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 1;
    while ans * 2 <= n {
        ans *= 2;
    }
    println!("{}", ans);
}
