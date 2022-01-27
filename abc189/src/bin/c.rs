use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.push(0);
    let mut ans = 0;
    // (高さ, l)を保存している
    let mut dq: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..=n {
        println!("{} {:?}", i, dq);
        let mut nl = i;
        while !dq.is_empty() && dq.back().unwrap().0 > a[i] {
            // キュー内でa[i]より大きいものは面積を計算して、キューから除外する
            let (h, l) = *dq.back().unwrap();
            println!("{} {}", h, l);
            ans = ans.max(h * (i - l));
            dq.pop_back();
            nl = l;
        }
        if dq.is_empty() || dq.back().unwrap().0 < a[i] {
            // 最大値の場合はキューに追加
            dq.push_back((a[i], nl));
        }
    }
    println!("{}", ans);
}
