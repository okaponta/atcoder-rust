use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        mut a:[[Usize1;n-1];n],
    }
    for i in 0..n {
        a[i].reverse();
    }
    let mut is_ready = vec![vec![false; n]; n];
    let mut cur = vec![];
    for i in 0..n {
        if let Some(j) = a[i].pop() {
            is_ready[i][j] = true;
            if is_ready[j][i] {
                cur.push(i);
                cur.push(j);
            }
        }
    }
    let mut ans = 0;
    while !cur.is_empty() {
        let mut next = vec![];
        for i in cur {
            if let Some(j) = a[i].pop() {
                is_ready[i][j] = true;
                if is_ready[j][i] {
                    next.push(i);
                    next.push(j);
                }
            }
        }
        cur = next;
        ans += 1;
    }
    println!(
        "{}",
        if a.iter().all(|i| i.len() == 0) {
            ans
        } else {
            -1
        }
    );
}
