use proconio::input;

fn main() {
    input! {
        n:i32,s:i32,d:i32,
        xy:[(i32,i32);n]
    }
    let mut ans = "No";
    for e in xy {
        if e.0 < s && e.1 > d {
            ans = "Yes"
        }
    }
    println!("{}", ans);
}
