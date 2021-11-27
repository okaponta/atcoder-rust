use proconio::input;

fn main() {
    input! {
        mut n: i32,
    }
    if n == 0 {
        println!("Yes");
        return;
    }
    while n % 10 == 0 {
        n /= 10;
    }
    let nstr = n.to_string();
    let rev = nstr.chars().rev().collect::<String>();
    println!("{}", if nstr == rev { "Yes" } else { "No" })
}
