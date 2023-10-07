use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
        s:[Chars;n],
    }
    let mut points = vec![];
    for i in 0..m {
        points.push((a[i], i));
    }
    points.sort();
    points.reverse();
    let mut player = vec![0; n];
    for i in 0..n {
        player[i] += i;
        for j in 0..m {
            if s[i][j] == 'o' {
                player[i] += a[j];
            }
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut need = 0;
        for j in 0..n {
            if i != j {
                need = need.max(player[j]);
            }
        }
        if need < player[i] {
            continue;
        }
        let mut tmp = 0;
        for j in 0..m {
            if s[i][points[j].1] == 'o' {
                continue;
            }
            tmp += points[j].0;
            ans[i] += 1;
            if need - player[i] < tmp {
                break;
            }
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
