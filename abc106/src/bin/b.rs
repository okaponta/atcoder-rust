use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
    }
    let eight = vec![105, 135, 165, 189, 195];
    println!("{}", eight.upper_bound(&n));
}
