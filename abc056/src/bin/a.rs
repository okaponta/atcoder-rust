use proconio::input;

fn main() {
    input! {
        a:char,
        b:char,
    }
    println!("{}", if a == b { 'H' } else { 'D' });
}
