use proconio::input;

fn main() {
    input! {
            x: i32,
            y: i32,
    }
    let ans;
    if x == y {
        ans = x;
    } else {
        ans = 3 - x - y;
    }
    println!("{}", ans)
}
