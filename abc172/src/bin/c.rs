use proconio::input;

fn main() {
    input! {
       n:usize,m:usize,k:i64,
       mut a:[i64;n],b:[i64;m],
    }
    a.insert(0, 0);
    let mut time = b.iter().sum::<i64>();
    let mut j = m;
    let mut ans = 0;

    for i in 0..=n {
        time += a[i];
        while time > k && j > 0 {
            j -= 1;
            time -= b[j];
        }
        if time > k {
            break;
        }
        ans = ans.max(i + j);
    }
    println!("{}", ans);
}
