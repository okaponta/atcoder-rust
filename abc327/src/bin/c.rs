use proconio::input;

fn main() {
    input! {
        a:[[usize;9];9],
    }
    let mut check = vec![vec![vec![]; 9]; 3];
    for i in 0..9 {
        for j in 0..9 {
            check[0][i].push(a[i][j]);
            check[1][j].push(a[i][j]);
            check[2][(i / 3) * 3 + (j / 3)].push(a[i][j]);
        }
    }
    for i in 0..9 {
        for j in 0..3 {
            check[j][i].sort();
            check[j][i].dedup();
            if check[j][i].len() != 9 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
