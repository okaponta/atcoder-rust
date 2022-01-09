use proconio::input;

fn main() {
    input! {
       n:usize,m:usize,x:i32,
       ca:[(i32,[i32;m]);n],
    }
    let mut ans = 1 << 30;
    for i in 0..1 << n {
        let mut books = vec![false; n];
        for j in 0..n {
            if i >> j & 1 == 1 {
                books[j] = true;
            }
        }
        let mut cost = 0;
        let mut skill = vec![0; m];
        for j in 0..n {
            if books[j] {
                cost += ca[j].0;
                for k in 0..m {
                    skill[k] += ca[j].1[k];
                }
            }
        }
        if skill.iter().filter(|e| e < &&x).count() == 0 {
            ans = ans.min(cost);
        }
    }
    println!("{}", if ans == 1 << 30 { -1 } else { ans });
}
