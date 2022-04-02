use proconio::input;

fn main() {
    input! {
        n:usize,mut k:usize,x:usize,
        mut a:[usize;n],
    }
    for i in 0..n {
        if k > 0 && a[i] >= x {
            let max = a[i] / x;
            let sub = max.min(k);
            a[i] -= sub * x;
            k -= sub;
        }
    }
    a.sort();
    let mut index = n;
    while k > 0 && index > 0 {
        index -= 1;
        a[index] = 0;
        k -= 1;
    }
    let ans = a.iter().sum::<usize>();
    println!("{}", ans);
}
