use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n:usize,
        k:usize,
        a:[Usize1;k],
    }
    if k % 2 == 0 {
        let ans = (0..k)
            .into_iter()
            .step_by(2)
            .map(|i| a[i + 1] - a[i])
            .sum::<usize>();
        println!("{}", ans);
        return;
    }
    if k == 1 {
        println!("0");
        return;
    }
    let mut odd = vec![0];
    for i in 0..k / 2 {
        odd.push(odd[i] + a[2 * i + 1] - a[2 * i]);
    }
    let mut even = vec![0];
    for i in 0..k / 2 {
        even.push(even[i] + a[2 * i + 2] - a[2 * i + 1]);
    }
    let mut ans = 1 << 60;
    for i in 0..=k / 2 {
        let tmp = odd[i] + even[k / 2] - even[i];
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
