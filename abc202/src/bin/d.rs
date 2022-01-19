use proconio::input;

fn main() {
    input! {
        mut a:usize, mut b:usize, mut k:usize
    }
    let binom = binom_init(a + b);
    let mut ans = vec![];
    for _ in 0..a + b {
        // a+b-1Ca-1 = a+b-1Cb
        let a_comb = binom[a + b - 1][b];
        if k <= a_comb {
            ans.push('a');
            a -= 1;
        } else {
            ans.push('b');
            b -= 1;
            k -= a_comb;
        }
    }
    println!("{}", ans.iter().collect::<String>());
}

fn binom_init(n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; n + 1]; n + 1];
    res[0][0] = 1;
    for i in 1..=n {
        res[i][0] = 1;
        res[i][i] = 1;
        for j in 1..i {
            res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
        }
    }
    res
}
