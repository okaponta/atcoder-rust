use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        p:[Usize1;n],
        c:[i64;n]
    }
    let mut ans: i64 = -1 << 60;
    for i in 0..n {
        let mut cur = p[i];
        let mut cost = vec![c[i]];
        let mut sum = c[i];
        while cur != i {
            cost.push(c[cur]);
            sum += c[cur];
            cur = p[cur];
        }
        let mut s = 0;
        for j in 0..cost.len() {
            if k <= j {
                break;
            }
            s += cost[j];
            let mut temp = s;
            if 0 < sum {
                temp += ((k - j - 1) / cost.len()) as i64 * sum;
            }
            ans = ans.max(temp);
        }
    }
    println!("{}", ans);
}
