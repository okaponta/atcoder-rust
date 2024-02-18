use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = (0..=n).into_iter().map(|_| "10").collect::<String>();
    ans.pop();
    println!("{}", ans);
}
