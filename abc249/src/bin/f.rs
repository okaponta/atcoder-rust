use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut k:usize,
        mut ty:[(u8,i64);n],
    }
    ty.insert(0, (1, 0));
    let mut p = 0;
    let mut heap = BinaryHeap::new();
    let mut ans = -1 << 60;
    for &(t, y) in ty.iter().rev() {
        if t == 1 {
            // 置き換えた値+これまでの累積
            ans = ans.max(p + y);
            if k == 0 {
                break;
            }
            // 無視した場合を考えるため1減算
            k -= 1;
        } else {
            if y < 0 {
                // できるだけ選びたくない
                heap.push(y);
            } else {
                p += y;
            }
        }
        while heap.len() > k {
            // 無視できないものだけ選択
            if let Some(m) = heap.pop() {
                p += m;
            }
        }
    }
    println!("{}", ans);
}
