use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[[u8;n];n],
        b:[[u8;n];n],
    }
    'a: for _ in 0..5 {
        a = rot(&a, n);
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 {
                    if b[i][j] != 1 {
                        continue 'a;
                    }
                }
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}

fn rot(a: &Vec<Vec<u8>>, n: usize) -> Vec<Vec<u8>> {
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            res[i][j] = a[n - 1 - j][i];
        }
    }
    res
}
