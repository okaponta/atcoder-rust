use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        k:usize,
        s:Chars,
    }
    let mut a = s.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a.push(('0', 0));
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut flip = 0;
    let mut count = 0;
    let mut ans = 0;
    loop {
        while flip <= k && right < a.len() {
            if a[right].0 == '0' {
                if flip == k {
                    break;
                }
                flip += 1;
            }
            count += a[right].1;
            right += 1;
        }
        ans = ans.max(count);
        if right == a.len() {
            break;
        }
        count -= a[left].1;
        left += 1;
        flip -= 1;
        if a[left].0 == '0' {
            count -= a[left].1;
            left += 1;
        }
    }
    println!("{}", ans);
}
