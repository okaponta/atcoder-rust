use proconio::input;

fn main() {
    input! {
       n:i32,d:i32,
       mut lr:[(i32,i32);n],
    }
    lr.sort_by_key(|&(_l, r)| r);
    let mut ans = 1;
    let mut punch = lr[0].1;
    for wall in lr {
        if wall.0 < punch + d {
            continue;
        } else {
            ans += 1;
            punch = wall.1;
        }
    }
    println!("{}", ans);
}
