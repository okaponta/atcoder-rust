use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut count = 0;
    while 1000 <= n {
        n /= 10;
        count += 1;
    }
    for _ in 0..count {
        n *= 10;
    }
    println!("{}", n);
}
