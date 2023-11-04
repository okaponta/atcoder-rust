use proconio::input;

fn main() {
    input! {
        a:[[usize;9];9],
    }
    let mut row = vec![vec![]; 9];
    let mut col = vec![vec![]; 9];
    let mut small = vec![vec![]; 9];
    for i in 0..9 {
        for j in 0..9 {
            row[i].push(a[i][j]);
            col[j].push(a[i][j]);
            small[(i / 3) * 3 + (j / 3)].push(a[i][j]);
        }
    }
    for i in 0..9 {
        row[i].sort();
        row[i].dedup();
        if row[i].len() != 9 {
            println!("No");
            return;
        }
        col[i].sort();
        col[i].dedup();
        if col[i].len() != 9 {
            println!("No");
            return;
        }

        small[i].sort();
        small[i].dedup();
        if small[i].len() != 9 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
