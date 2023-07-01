use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        s:Chars,
    }
    let mut mdp = vec![0; 3];
    let mut edp = vec![0; 6];
    let mut ans = 0usize;
    for i in 0..n {
        if s[i] == 'M' {
            mdp[a[i]] += 1;
        } else if s[i] == 'E' {
            if a[i] == 0 {
                edp[0] += mdp[0];
                edp[1] += mdp[1];
                edp[2] += mdp[2];
            } else if a[i] == 1 {
                edp[1] += mdp[0];
                edp[3] += mdp[1];
                edp[4] += mdp[2];
            } else {
                edp[2] += mdp[0];
                edp[4] += mdp[1];
                edp[5] += mdp[2];
            }
        } else {
            if a[i] == 0 {
                ans += edp[0];
                ans += 2 * edp[1];
                ans += edp[2];
                ans += 2 * edp[3];
                ans += 3 * edp[4];
                ans += edp[5];
            } else if a[i] == 1 {
                ans += 2 * edp[0];
                ans += 2 * edp[1];
                ans += 3 * edp[2];
            } else {
                ans += edp[0];
                ans += 3 * edp[1];
                ans += edp[2];
            }
        }
    }
    println!("{}", ans);
}
