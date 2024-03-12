use proconio::input;

fn main() {
    input! {
        n:usize,
        w:[usize;n],
        b:[usize;n],
    }
    let mut grundy = vec![vec![0; 1500]; 51];
    for i in 0..51 {
        for j in 0..1300 {
            let mut mex = vec![false; 1500];
            if 0 < i {
                mex[grundy[i - 1][j + i]] = true;
            }
            if 0 < j {
                for k in 1..=(j / 2) {
                    mex[grundy[i][j - k]] = true;
                }
            }
            grundy[i][j] = calc_mex(mex);
        }
    }
    let xor = (0..n).fold(0, |s, i| s ^ grundy[w[i]][b[i]]);
    println!("{}", if xor == 0 { "Second" } else { "First" });
}

fn calc_mex(vec: Vec<bool>) -> usize {
    for k in 0..1500 {
        if !vec[k] {
            return k;
        }
    }
    return 1500;
}
