use proconio::input;

fn main() {
    input! {
        abc:[usize;3],
    }
    println!("{}", abc[0] * abc[1] / 2);
}
