use proconio::input;

fn main() {
    input! {
       s:[String;2],
    }
    let s0 = &s[0][..];
    let s1 = &s[1][..];
    let mut ans = true;
    match s0 {
        ".#" => {
            if s1 == "#." {
                ans = false
            }
        }
        "#." => {
            if s1 == ".#" {
                ans = false
            }
        }
        _ => {}
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
