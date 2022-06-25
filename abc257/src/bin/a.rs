use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
    }
    let idx = (x - 1) / n;
    println!("{}", (b'A' + idx as u8) as char);
}
