use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let ok = s.iter().filter(|&s| s == "For").count();
    println!("{}", if n / 2 < ok { "Yes" } else { "No" });
}
