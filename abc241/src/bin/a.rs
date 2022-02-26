use proconio::input;

fn main() {
    input! {
        a:[usize;10],
    }
    let ans = a[a[a[0]]];
    println!("{}", ans);
}
