use proconio::input;

fn main() {
    input! {
        mut n:[usize;3],
    }
    n.sort();
    println!("{}", n[0] + n[1]);
}
