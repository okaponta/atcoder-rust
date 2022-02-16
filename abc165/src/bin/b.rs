use proconio::input;

fn main() {
    input! {
       x:i128,
    }
    let mut m = 100;
    let mut count = 0;
    while m < x {
        m = m * 101 / 100;
        count += 1;
    }
    println!("{}", count);
}
