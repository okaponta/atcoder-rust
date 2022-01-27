use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        ab:[(Usize1,Usize1);m],
        k:usize,
        cd:[(Usize1,Usize1);k],
    }
    let mut ans = 0;
    // 2^kのbit全探索
    for i in 0..1 << k {
        let mut count = vec![0; n];
        for j in 0..k {
            if i & (1 << j) > 0 {
                // フラグが立ってたら前者を採用
                count[cd[j].0] += 1;
            } else {
                count[cd[j].1] += 1;
            }
        }
        let mut cand = 0;
        for (a, b) in &ab {
            if count[*a] > 0 && count[*b] > 0 {
                cand += 1;
            }
        }
        ans = ans.max(cand);
    }
    println!("{}", ans);
}
