use proconio::input;

fn main() {
    input! {
        a:i32,
        o:char,
        b:i32,
    }
    println!("{}", if o == '+' { a + b } else { a - b });
}
