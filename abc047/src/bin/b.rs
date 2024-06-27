use proconio::input;

fn main() {
    input! {
        w:usize,
        h:usize,
        n:usize,
        xya:[(usize,usize,u8);n],
    }
    let mut grid = vec![vec![true; h]; w];
    for (x, y, a) in xya {
        let mut lx = (0, w);
        let mut ly = (0, h);
        if a == 1 {
            lx.1 = x;
        } else if a == 2 {
            lx.0 = x;
        } else if a == 3 {
            ly.1 = y;
        } else {
            ly.0 = y;
        }
        for i in lx.0..lx.1 {
            for j in ly.0..ly.1 {
                grid[i][j] = false;
            }
        }
    }
    let mut ans = 0;
    for i in 0..w {
        for j in 0..h {
            if grid[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
