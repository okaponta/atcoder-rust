use proconio::input;

fn main() {
    input! {
       n:usize,
       x:[isize;n]
    }
    let m = x.iter().fold(0, |acc, x| acc + x.abs());
    let e = (x.iter().fold(0, |acc, x| acc + x * x) as f64).sqrt();
    let c = x.iter().map(|x| x.abs()).max().unwrap();
    println!("{}", m);
    println!("{}", e);
    println!("{}", c);
}
