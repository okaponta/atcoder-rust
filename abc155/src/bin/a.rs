use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,c:i32,
    }
    if a == b && b == c {
        println!("No");
        return;
    }
    if a == b || b == c || c == a {
        println!("Yes");
        return;
    }
    println!("No");
}
