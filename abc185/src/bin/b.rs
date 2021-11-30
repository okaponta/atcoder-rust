use proconio::input;

fn main() {
    input! {
        n:i32,m:i32,t:i32,
        ab:[(i32,i32);m]
    }
    let mut bat = n;
    let mut time = 0;
    for &(a, b) in &ab {
        let bef = bat - (a - time);
        if bef <= 0 {
            println!("No");
            return;
        }
        bat = (bef + (b - a)).min(n);
        time = b;
    }
    println!("{}", if bat <= (t - time) { "No" } else { "Yes" });
}
