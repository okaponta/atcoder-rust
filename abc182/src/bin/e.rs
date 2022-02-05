use proconio::{input, marker::Usize1};

fn main() {
    input! {
       h:usize,w:usize,
       n:usize,m:usize,
       ab:[(Usize1,Usize1);n],
       cd:[(Usize1,Usize1);m],
    }
    let mut ans = vec![vec![false; w]; h];
    let mut lights = vec![vec![false; w]; h];
    let mut blocks = vec![vec![false; w]; h];
    for (a, b) in ab {
        lights[a][b] = true;
    }
    for (c, d) in cd {
        blocks[c][d] = true;
    }
    for i in 0..h {
        let mut flag = false;
        for j in 0..w {
            if lights[i][j] {
                ans[i][j] = true;
                flag = true;
            } else if blocks[i][j] {
                flag = false;
            } else {
                if flag {
                    ans[i][j] = true;
                }
            }
        }
    }
    for i in 0..h {
        let mut flag = false;
        for j in (0..w).rev() {
            if lights[i][j] {
                ans[i][j] = true;
                flag = true;
            } else if blocks[i][j] {
                flag = false;
            } else {
                if flag {
                    ans[i][j] = true;
                }
            }
        }
    }
    for j in 0..w {
        let mut flag = false;
        for i in 0..h {
            if lights[i][j] {
                ans[i][j] = true;
                flag = true;
            } else if blocks[i][j] {
                flag = false;
            } else {
                if flag {
                    ans[i][j] = true;
                }
            }
        }
    }
    for j in 0..w {
        let mut flag = false;
        for i in (0..h).rev() {
            if lights[i][j] {
                ans[i][j] = true;
                flag = true;
            } else if blocks[i][j] {
                flag = false;
            } else {
                if flag {
                    ans[i][j] = true;
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
