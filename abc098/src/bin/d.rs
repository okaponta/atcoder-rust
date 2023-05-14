use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut left = 0;
    let mut right = 0;
    let mut xor = 0;
    let mut sum = 0;
    let mut ans = 0;
    loop {
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
        if left == n - 1 {
            // しゃくとり終了
            break;
        }
        left += 1;
    }
    println!("{}", ans);
}
