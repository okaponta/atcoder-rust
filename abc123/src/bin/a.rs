use proconio::input;

fn main() {
    input! {
        mut a:[usize;5],
        k:usize
    }
    a.sort();
    println!("{}", if a[4] - a[0] <= k { "Yay!" } else { ":(" });
}
