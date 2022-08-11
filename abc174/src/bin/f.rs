use num_integer::Roots;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
       n:usize,
       q:usize,
       c:[usize;n],
       lr:[(Usize1,usize);q],
    }
    let mut query = vec![];
    let blocks = n.sqrt(); // ブロックの数
    for (i, &(l, r)) in lr.iter().enumerate() {
        query.push((i, l, r, l / blocks)); // インデックス/l/r/ブロックの番号
    }
    // ブロック順、同じブロックはr順でソート
    query.sort_by(|a, b| a.3.cmp(&b.3).then(a.2.cmp(&b.2)));

    let mut ans_arr = vec![];

    let mut count = vec![0; n + 1];
    let mut l_tmp = 0;
    let mut r_tmp = 0;
    let mut ans = 0;
    for (i, l, r, _) in query {
        // lが大きければ減少させる(範囲拡張)
        while l_tmp > l {
            l_tmp -= 1;
            if count[c[l_tmp]] == 0 {
                ans += 1;
            }
            count[c[l_tmp]] += 1;
        }
        // rが小さければ増加させる(範囲拡張)
        while r_tmp < r {
            if count[c[r_tmp]] == 0 {
                ans += 1;
            }
            count[c[r_tmp]] += 1;
            r_tmp += 1;
        }
        // lが小さければ増加させる(範囲縮小)
        while l_tmp < l {
            count[c[l_tmp]] -= 1;
            if count[c[l_tmp]] == 0 {
                ans -= 1;
            }
            l_tmp += 1;
        }
        // rが大きければ減少させる(範囲縮小)
        while r_tmp > r {
            r_tmp -= 1;
            count[c[r_tmp]] -= 1;
            if count[c[r_tmp]] == 0 {
                ans -= 1;
            }
        }
        ans_arr.push((i, ans));
    }
    ans_arr.sort();
    for r in ans_arr {
        println!("{}", r.1);
    }
}
