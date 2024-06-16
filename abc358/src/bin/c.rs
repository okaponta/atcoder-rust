use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    let mut ans = 100;
    for i in 1..1 << n {
        let mut pop = vec![false; m];
        let mut count = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                count += 1;
                for k in 0..m {
                    if s[j][k] == 'o' {
                        pop[k] = true;
                    }
                }
            }
        }
        if pop.iter().all(|&b| b) {
            ans = ans.min(count);
        }
    }
    println!("{}", ans);
}
