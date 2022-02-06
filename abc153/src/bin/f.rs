use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,d:usize,a:usize,
        mut xh:[(usize,usize);n],
    }
    xh.sort();
    let mut imos: VecDeque<(usize, usize)> = VecDeque::new();
    let mut damage = 0;
    let mut ans = 0;
    for (x, h) in xh {
        // 爆風が到達していないぶんの処理
        while !imos.is_empty() && imos.front().unwrap().0 < x {
            let less = imos.pop_front().unwrap();
            damage -= less.1 * a;
        }
        if damage >= h {
            // 爆風で倒せる
            continue;
        }
        // 爆弾を投げる場合
        let times = (h - damage + a - 1) / a;
        ans += times;
        damage += times * a;
        imos.push_back((x + 2 * d, times));
    }
    println!("{}", ans);
}
