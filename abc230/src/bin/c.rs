use proconio::{input, marker::Usize1};

fn main() {
    input! {
       _n:usize,a:Usize1,b:Usize1,
       p:Usize1,q:Usize1,r:Usize1,s:Usize1,
    }
    let mut first = b as i64 - a as i64 + p as i64;
    let mut second = (b + a) as i64 - p as i64;
    for _i in p..=q {
        let mut ans = vec!['.'; s - r + 1];
        if r as i64 <= first && first <= s as i64 {
            ans[first as usize - r] = '#';
        }
        if r as i64 <= second && second <= s as i64 {
            ans[second as usize - r] = '#';
        }
        println!("{}", ans.iter().collect::<String>());
        first += 1;
        second -= 1;
    }
}
