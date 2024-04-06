use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    for i in 0..n {
        print!("{}", if i % 3 == 2 { 'x' } else { 'o' });
    }
    println!();
}
