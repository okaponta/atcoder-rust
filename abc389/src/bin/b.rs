use proconio::*;

fn main() {
    input! {
        mut x:usize,
    }
    for i in 1..99 {
        x /= i;
        if x == 1 {
            println!("{i}");
            return;
        }
    }
}
