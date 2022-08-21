use proconio::input;

fn main() {
    input! {
        x:usize,
        y:usize,
        n:usize,
    }
    if x * 3 > y {
        println!("{}", (n / 3) * y + (n % 3) * x);
    } else {
        println!("{}", x * n);
    }
}
