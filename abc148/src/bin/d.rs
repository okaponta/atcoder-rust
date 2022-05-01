use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    let mut count = 1;
    for ai in a {
        if ai == count {
            count += 1;
        } else {
            ans += 1;
        }
    }
    println!("{}", if ans == n { -1 } else { ans as i32 });
}
