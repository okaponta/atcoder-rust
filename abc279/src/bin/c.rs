use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
        t:[Chars;h],
    }
    let mut st = vec![vec![]; w];
    let mut tt = vec![vec![]; w];
    for i in 0..h {
        for j in 0..w {
            st[j].push(s[i][j]);
            tt[j].push(t[i][j]);
        }
    }
    st.sort();
    tt.sort();
    for i in 0..w {
        if st[i] != tt[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
