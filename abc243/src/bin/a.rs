use proconio::input;

fn main() {
    input! {
        mut v:i32,a:i32,b:i32,c:i32,
    }
    v %= a + b + c;
    if v - a < 0 {
        println!("F");
    } else if v - a - b < 0 {
        println!("M");
    } else {
        println!("T");
    }
}
