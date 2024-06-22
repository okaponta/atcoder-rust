use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut ans = 0;
    for s in s {
        if &s == "Takahashi" {
            ans += 1;
        }
    }
    println!("{}", ans);
}
