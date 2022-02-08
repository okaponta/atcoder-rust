use proconio::input;

fn main() {
    input! {
       mut x:u128,y:u128,a:u128,b:u128,
    }
    let mut ans = 0;
    while a * x < b + x {
        x *= a;
        ans += 1;
        if y <= x {
            println!("{}", ans - 1);
            return;
        }
    }
    let p = (y - x + b - 1) / b;
    println!("{}", ans + p - 1);
}
