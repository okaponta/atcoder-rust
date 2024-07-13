use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s:Chars,
        q:usize,
        mut t:[Chars;q],
    }
    let sa = suffix_array(&s);
    for mut t in t {
        let a = sa_lower_bound(&s, &t, &sa);
        t.push('~');
        let b = sa_lower_bound(&s, &t, &sa);
        println!("{}", b - a);
    }
}

// 接尾辞配列を構成する
fn suffix_array(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut sa = vec![0; n + 1];
    let mut rank = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    for i in 0..=n {
        sa[i] = i;
        rank[i] = if i < n { s[i] as i32 } else { -1 };
    }

    fn compare_sa(i: usize, j: usize, k: usize, n: usize, rank: &Vec<i32>) -> std::cmp::Ordering {
        if rank[i] != rank[j] {
            return rank[i].cmp(&rank[j]);
        }
        let ri = if i + k <= n { rank[i + k] } else { -1 };
        let rj = if j + k <= n { rank[j + k] } else { -1 };
        ri.cmp(&rj)
    }

    let mut k = 1;
    while k <= n {
        sa.sort_by(|&a, &b| compare_sa(a, b, k, n, &rank));
        tmp[sa[0]] = 0;
        for i in 1..=n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if compare_sa(sa[i - 1], sa[i], k, n, &rank) == std::cmp::Ordering::Less {
                    1
                } else {
                    0
                };
        }
        for i in 0..=n {
            rank[i] = tmp[i];
        }
        k *= 2;
    }
    sa
}

fn sa_lower_bound(s: &Vec<char>, t: &Vec<char>, sa: &Vec<usize>) -> usize {
    let mut lower = 0;
    let mut upper = s.len() + 1;
    let n = t.len();
    while 1 < upper - lower {
        let mid = (lower + upper) / 2;
        if t[0..n] <= s[sa[mid]..] {
            upper = mid;
        } else {
            lower = mid;
        }
    }
    lower
}
