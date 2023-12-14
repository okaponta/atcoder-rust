use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    if 9 < a + b {
        println!("error");
    } else {
        println!("{}", a + b);
    }
}
