use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:[Chars;9],
    }
    let mut set = HashSet::new();
    for i in 0..81 {
        if s[i / 9][i % 9] == '.' {
            continue;
        }
        for j in i + 1..81 {
            if s[j / 9][j % 9] == '.' {
                continue;
            }
            let i = i as i32;
            let ix = i / 9;
            let iy = i % 9;
            let j = j as i32;
            let jx = j / 9;
            let jy = j % 9;
            if iy == jy {
                continue;
            }
            let vx = (jy - iy).abs();
            let vy = if ix == jx {
                0
            } else {
                vx * (jx - ix) / (iy - jy)
            };
            let kx = ix + vx;
            let ky = iy + vy;
            let lx = jx + vx;
            let ly = jy + vy;
            if 0 <= kx && kx < 9 && 0 <= ky && ky < 9 && s[kx as usize][ky as usize] == '#' {
                if 0 <= lx && lx < 9 && 0 <= ly && ly < 9 && s[lx as usize][ly as usize] == '#' {
                    let mut a = vec![];
                    a.push(ix * 9 + iy);
                    a.push(jx * 9 + jy);
                    a.push(kx * 9 + ky);
                    a.push(lx * 9 + ly);
                    a.sort();
                    a.dedup();
                    if a.len() == 4 {
                        set.insert(a);
                    }
                }
            }
        }
    }
    println!("{}", set.len());
}
