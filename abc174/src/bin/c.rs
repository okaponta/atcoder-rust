use proconio::input;

fn main() {
    input! {
       k:i32,
    }
    if k == 1 || k == 7 {
        println!("{}", 1);
        return;
    }
    let mut st = 7;
    let mut rem = 7;
    for i in 2..=k {
        st = st * 10 % k;
        rem = (rem + st) % k;
        if rem == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
