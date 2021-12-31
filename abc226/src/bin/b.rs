use proconio::input;

fn main() {
    input! {
       n:i32, mut la:[[i32];n]
    }
    la.sort();
    la.dedup();
    println!("{}", la.len());
}
