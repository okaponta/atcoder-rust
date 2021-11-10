use proconio::input;

fn main() {
    input! {
            n: usize,
            a: [usize;n],
    }
    let mut ans = 0;
    for e in &a {
        if e > &10 {
            ans += e - 10;
        }
    }
    println!("{}", ans)
}
