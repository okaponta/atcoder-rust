use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    if a < b && b < c {
        println!("Yes");
    } else if c < b && b < a {
        println!("Yes");
    } else if a == b || b == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
