use proconio::input;

fn main() {
    input! {
            a: i32,
            b: i32,
            c: i32,
    }
    if (a + b + c) % 3 != 0 {
        println!("No");
        return;
    }
    let ave = (a + b + c) / 3;
    if a == ave || b == ave || c == ave {
        println!("Yes");
        return;
    }
    println!("No");
}
