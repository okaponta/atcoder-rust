use proconio::input;

fn main() {
    input! {
        mut n:usize,
    }
    if n % 2 == 1 {
        println!("0");
        return;
    }
    let mut ans = 0;
    n /= 2;
    while 0 < n {
        n /= 5;
        ans += n;
    }
    println!("{}", ans);
}
