use proconio::input;

fn main() {
    input! {
        x:usize,
        y:usize,
    }
    println!("{}", ((y / x) + 1).next_power_of_two().trailing_zeros());
}
