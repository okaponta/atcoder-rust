use proconio::input;

fn main() {
    input! {
        x1:i32,
        y1:i32,
        x2:i32,
        y2:i32,
    }
    let x = x2 - x1;
    let y = y2 - y1;
    let x3 = x2 - y;
    let y3 = y2 + x;
    let x4 = x3 - x;
    let y4 = y3 - y;
    println!("{} {} {} {}", x3, y3, x4, y4);
}
