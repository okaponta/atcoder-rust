use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    let mut count = 0;
    loop {
        while sum < k && right < n {
            sum += a[right];
            right += 1;
        }
        if sum < k {
            // しゃくとり終了
            break;
        }
        count += n + 1 - right;
        sum -= a[left];
        left += 1;
    }
    println!("{}", count);
}
