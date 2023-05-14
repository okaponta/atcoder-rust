use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut right = 0;
    let mut xor = 0;
    let mut sum = 0;
    let mut ans = 0;
    for left in 0..n {
        while sum == xor && right < n {
            sum += a[right];
            xor ^= a[right];
            if sum == xor {
                right += 1;
            } else {
                sum -= a[right];
                xor ^= a[right];
                break;
            }
        }
        ans += right - left;
        sum -= a[left];
        xor ^= a[left];
    }
    println!("{}", ans);
}
