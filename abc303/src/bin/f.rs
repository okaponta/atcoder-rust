use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut h:usize,
        td:[(usize,usize);n],
    }
    let mut map = BTreeMap::new();
    for &(t, d) in &td {
        map.insert(t, *map.get(&t).unwrap_or(&0).max(&d));
    }
    // t, d, maxtd, maxd
    let mut magics = map
        .into_iter()
        .map(|(k, v)| (k, v, 0, 0))
        .collect::<Vec<_>>();
    let n = magics.len();
    let mut max = 0;
    for i in (0..n).rev() {
        max = max.max(magics[i].1);
        magics[i].3 = max;
    }
    max = 0;
    for i in 0..n {
        magics[i].2 = max;
        max = max.max(magics[i].0 * magics[i].1);
    }

    let mut turn = 0;
    for (t, _, maxtd, maxd) in magics {
        // 区間でのダメージ
        let cross = (maxtd + maxd - 1) / maxd;
        let damage = calc_damage(turn, cross, t, maxd, maxtd);
        if damage < h {
            h -= damage;
            turn = t;
            continue;
        }
        // 二分探索で求める
        let mut lower = turn;
        let mut upper = t;
        while 1 < upper - lower {
            let mid = (upper + lower) / 2;
            if calc_damage(turn, cross, mid, maxd, maxtd) < h {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);
        return;
    }
    // 決着がつかなかったら、maxtdをひいていく
    println!("{}", turn - 1 + (h + max - 1) / max);
}

fn calc_damage(turn: usize, cross: usize, t: usize, maxd: usize, maxtd: usize) -> usize {
    if cross < turn {
        return (turn + t - 1) * maxd * (t - turn) / 2;
    }
    if t < cross {
        return (t - turn) * maxtd;
    }
    return (cross - turn) * maxtd + ((cross + t - 1) * maxd * (t - cross) / 2);
}
