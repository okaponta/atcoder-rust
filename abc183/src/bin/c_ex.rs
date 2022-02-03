use itertools::Itertools;

fn main() {
    proconio::input! {
      n:usize,k:usize,
      t:[[usize;n];n],
    }
    let mut ans = 0;
    for i in (1..n).permutations(n - 1) {
        let a = t[0][i[0]] + t[i[n - 2]][0] + i.windows(2).map(|j| t[j[0]][j[1]]).sum::<usize>();
        if a == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
