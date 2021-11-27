use proconio::input;

fn main() {
    input! {
       n:usize,x:i32,
       mut a: [i32;n]
    }
    for i in 0..n {
        if a[i] != x {
            print!("{} ", a[i]);
        }
    }
    println!();
}
