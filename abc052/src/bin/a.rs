use proconio::input;

fn main() {
    input! {
        n:[usize;4],
    }
    println!("{}", (n[0] * n[1]).max(n[2] * n[3]));
}
