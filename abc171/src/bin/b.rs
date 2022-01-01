use proconio::input;

fn main() {
    input! {
       n:usize,k:usize,mut p:[i32;n]
    }
    p.sort();
    println!("{}", p[0..k].iter().sum::<i32>());
}
