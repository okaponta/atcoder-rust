use proconio::input;

fn main() {
    input! {
        lr:[(i32,i32);2],
    }
    println!("{}", (lr[0].1.min(lr[1].1) - lr[0].0.max(lr[1].0)).max(0));
}
