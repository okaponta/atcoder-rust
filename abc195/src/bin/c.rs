use proconio::input;

fn main() {
    input! {
       mut n:usize,
    }
    let mut bound = 999;
    let mut ans = 0;
    while n > bound {
        n -= bound;
        ans += n;
        bound *= 1000;
    }
    println!("{}", ans);
}
