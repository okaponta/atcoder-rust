use proconio::input;

fn main() {
    input! {
        n:u8,
    }
    println!("{}", (b'a' + (n - 97)) as char);
}
