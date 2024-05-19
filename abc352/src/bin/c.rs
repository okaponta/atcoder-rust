use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
    }
    let mut head = 0;
    let mut ans = 0;
    for (a, b) in ab {
        ans += a;
        head = head.max(b - a);
    }
    ans += head;
    println!("{}", ans);
}
