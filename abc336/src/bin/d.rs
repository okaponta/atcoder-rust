use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let mut left = vec![1];
    for i in 1..n {
        left.push((left[i - 1] + 1).min(a[i]));
    }
    a.reverse();
    let mut right = vec![1];
    for i in 1..n {
        right.push((right[i - 1] + 1).min(a[i]));
    }
    right.reverse();
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(left[i].min(right[i]));
    }
    println!("{}", ans);
}
