use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut ans = vec![1i64; 2];
    for si in s {
        if si == "AND" {
            ans[1] += ans[0] + ans[1];
        } else {
            ans[0] += ans[0] + ans[1];
        }
    }
    println!("{}", ans[0]);
}
