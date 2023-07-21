use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut price = vec![0; n];
    let mut fnum = vec![0; n];
    let mut func = vec![vec![false; m]; n];
    let mut ff = vec![];
    for i in 0..n {
        input! {
            p:usize,
            c:usize,
            f:[Usize1;c],
        }
        price[i] = p;
        fnum[i] = c;
        for k in 0..c {
            func[i][f[k]] = true;
        }
        ff.push(f);
        for j in 0..i {
            if p < price[j] || (p == price[j] && fnum[j] < c) {
                if (0..fnum[j]).into_iter().all(|k| func[i][ff[j][k]]) {
                    println!("Yes");
                    return;
                }
            } else if price[j] < p || (p == price[j] && c < fnum[j]) {
                if (0..c).into_iter().all(|k| func[j][ff[i][k]]) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
