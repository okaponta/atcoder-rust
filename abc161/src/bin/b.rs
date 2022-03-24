use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    let all = a.iter().sum::<usize>();
    let is_ok = a[m - 1] * 4 * m >= all;
    println!("{}", if is_ok { "Yes" } else { "No" });
}
