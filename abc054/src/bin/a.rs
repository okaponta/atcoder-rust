use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    let a = (a + 11) % 13;
    let b = (b + 11) % 13;
    if a < b {
        println!("Bob");
    } else if a == b {
        println!("Draw");
    } else {
        println!("Alice");
    }
}
