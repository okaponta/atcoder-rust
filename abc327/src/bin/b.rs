use proconio::input;

fn main() {
    input! {
        b:usize,
    }
    let mut a = 1usize;
    while a.pow(a as u32) <= b {
        if a.pow(a as u32) == b {
            println!("{}", a);
            return;
        }
        a += 1;
    }
    println!("-1");
}
