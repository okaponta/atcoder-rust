use proconio::input;

fn main() {
    input! {
       n:usize,mut k:i64,
       mut a:[i64;n]
    }
    a.push(0);
    a.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    let mut count = 1;
    for i in 1..n + 1 {
        let until_next = a[i - 1] - a[i];
        if k <= until_next * count {
            let quo = k / count;
            let rem = k % count;
            let all = a[i - 1] - quo + 1;
            ans += (all + a[i - 1]) * quo * count / 2;
            ans += rem * (all - 1);
            break;
        }
        k -= until_next * count;
        ans += (a[i - 1] + a[i] + 1) * until_next * count / 2;
        count += 1;
    }
    println!("{}", ans);
}
