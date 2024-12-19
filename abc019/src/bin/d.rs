use proconio::*;

fn main() {
    input_interactive!(n: usize);
    let mut max = 0;
    let mut pair = 0;
    for i in 1..n {
        println!("? 1 {}", i + 1);
        input_interactive!(d: usize);
        if max < d {
            max = d;
            pair = i;
        }
    }
    for i in 0..n {
        println!("? {} {}", pair + 1, i + 1);
        input_interactive!(d: usize);
        if max < d {
            max = d;
        }
    }
    println!("! {}", max);
}
