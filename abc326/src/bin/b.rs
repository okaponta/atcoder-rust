use proconio::input;

fn main() {
    input! {
        mut n:usize,
    }
    loop {
        let a = n / 100;
        let b = (n - a * 100) / 10;
        let c = n % 10;
        if a * b == c {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}
