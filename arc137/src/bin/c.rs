use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let mut ans = 0;
    let mut terminal = 0;
    a.sort();
    for i in 0..n {
        ans = ans ^ a[i];
        terminal = terminal ^ i;
    }
    println!("{}", if ans == terminal { "Bob" } else { "Alice" });
}
