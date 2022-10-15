use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        rs:Usize1,
        cs:Usize1,
        n:usize,
        rc:[(Usize1,Usize1);n],
        q:usize,
        dl:[(char,usize);q],
    }
    let mut i_set = BTreeSet::new();
    let mut j_set = BTreeSet::new();
    for (r, c) in rc {
        i_set.insert((r, c));
        j_set.insert((c, r));
    }
    let mut cur_r = rs;
    let mut cur_c = cs;
    for (d, l) in dl {
        // LRはj方向の移動
        if d == 'L' {
            if let Some((_, j)) = i_set.range((cur_r, 0)..(cur_r, cur_c)).last() {
                // かべあり
                cur_c = cur_c.saturating_sub(l).max(j + 1);
            } else {
                cur_c = cur_c.saturating_sub(l)
            }
        }
        if d == 'R' {
            if let Some((_, j)) = i_set.range((cur_r, cur_c)..(cur_r, w)).next() {
                // かべあり
                cur_c = (cur_c + l).min(j - 1);
            } else {
                cur_c = (cur_c + l).min(w - 1);
            }
        }
        // UDはi方向の移動
        if d == 'U' {
            if let Some((_, i)) = j_set.range((cur_c, 0)..(cur_c, cur_r)).last() {
                // かべあり
                cur_r = cur_r.saturating_sub(l).max(i + 1);
            } else {
                cur_r = cur_r.saturating_sub(l)
            }
        }
        if d == 'D' {
            if let Some((_, i)) = j_set.range((cur_c, cur_r)..(cur_c, h)).next() {
                // かべあり
                cur_r = (cur_r + l).min(i - 1);
            } else {
                cur_r = (cur_r + l).min(h - 1);
            }
        }
        println!("{} {}", cur_r + 1, cur_c + 1);
    }
}
