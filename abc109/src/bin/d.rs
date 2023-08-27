use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        mut a:[[u8;w];h],
    }
    let mut ans = vec![];
    for i in 0..h {
        for j in 0..w - 1 {
            if a[i][j] % 2 == 1 {
                ans.push((i, j, i, j + 1));
                a[i][j + 1] += 1;
            }
        }
    }
    for i in 0..h - 1 {
        if a[i][w - 1] % 2 == 1 {
            ans.push((i, w - 1, i + 1, w - 1));
            a[i + 1][w - 1] += 1;
        }
    }
    println!("{}", ans.len());
    for (a, b, c, d) in ans {
        println!("{} {} {} {}", a + 1, b + 1, c + 1, d + 1);
    }
}
