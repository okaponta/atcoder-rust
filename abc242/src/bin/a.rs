use proconio::input;

fn main() {
    input! {
        a:f64,b:f64,c:f64,x:f64,
    }
    if x <= a {
        println!("{}", 1.0f64);
        return;
    }
    if x <= b {
        println!("{}", c / (b - a));
        return;
    }
    println!("{}", 0.0f64);
}
