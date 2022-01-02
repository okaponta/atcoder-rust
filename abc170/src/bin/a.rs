use proconio::input;

fn main() {
    input! {
       x:[i32;5],
    }
    for i in 0..5 {
        if x[i] != (i + 1) as i32 {
            println!("{}", i + 1);
        }
    }
}
