use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
       r:i64,x:i64,y:i64,
    }
    if x * x + y * y < r * r {
        println!("{}", 2);
        return;
    }
    let mut ans = ((x * x + y * y) / (r * r)).sqrt();
    if x * x + y * y != ans * ans * r * r {
        ans += 1;
    }
    println!("{}", ans);
}
