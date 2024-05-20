use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let s = a.iter().sum::<usize>();
    if s % 10 != 0 {
        println!("No");
        return;
    }
    for i in 0..n {
        a.push(a[i]);
    }
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    loop {
        while sum < s / 10 && left < n {
            sum += a[right];
            right += 1;
        }
        if left == n {
            // しゃくとり終了
            break;
        }
        if sum == s / 10 {
            println!("Yes");
            return;
        }
        sum -= a[left];
        left += 1;
    }
    println!("No");
}
