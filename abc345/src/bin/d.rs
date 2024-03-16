use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n:usize,
        h:usize,
        w:usize,
        mut ab:[(usize,usize);n],
    }
    ab.sort_by_key(|(a, b)| a * b);
    ab.reverse();
    let mut set = HashSet::new();
    for i in 1..1 << n {
        let mut sum = 0;
        let mut v = vec![];
        for j in 0..n {
            if i >> j & 1 == 1 {
                sum += ab[j].0 * ab[j].1;
                v.push(ab[j]);
            }
        }
        if sum != h * w {
            continue;
        } else {
            if set.contains(&v) {
                continue;
            }
            let mut q = VecDeque::new();
            q.push_back((vec![vec![false; w]; h], 0));
            while let Some((t, index)) = q.pop_front() {
                let aa = v[index].0;
                let bb = v[index].1;
                if aa <= h && bb <= w {
                    for i in 0..=h - aa {
                        for j in 0..=w - bb {
                            if all_false(i, i + aa, j, j + bb, &t) {
                                let mut tt = t.clone();
                                for iii in i..i + aa {
                                    for jjj in j..j + bb {
                                        tt[iii][jjj] = true;
                                    }
                                }
                                if index + 1 == v.len() {
                                    println!("Yes");
                                    return;
                                } else {
                                    q.push_back((tt, index + 1));
                                }
                            }
                        }
                    }
                }
                if aa != bb && bb <= h && aa <= w {
                    for i in 0..=h - bb {
                        for j in 0..=w - aa {
                            if all_false(i, i + bb, j, j + aa, &t) {
                                let mut tt = t.clone();
                                for iii in i..i + bb {
                                    for jjj in j..j + aa {
                                        tt[iii][jjj] = true;
                                    }
                                }
                                if index + 1 == v.len() {
                                    println!("Yes");
                                    return;
                                } else {
                                    q.push_back((tt, index + 1));
                                }
                            }
                        }
                    }
                }
            }
            set.insert(v);
        }
    }
    println!("No");
}

fn all_false(i: usize, ii: usize, j: usize, jj: usize, t: &Vec<Vec<bool>>) -> bool {
    for iii in i..ii {
        for jjj in j..jj {
            if t[iii][jjj] {
                return false;
            }
        }
    }
    true
}
