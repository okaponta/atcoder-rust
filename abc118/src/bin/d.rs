use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
    }
    let num = vec![0, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut dp = vec![vec![0; 10]; n + 10];
    for i in 0..n {
        if i != 0 && dp[i] == vec![0; 10] {
            continue;
        }
        for j in 0..m {
            let mut tmp = dp[i].clone();
            tmp[a[j]] += 1;
            if cmp(&tmp, &dp[i + num[a[j]]]) {
                dp[i + num[a[j]]] = tmp;
            }
        }
    }
    for i in (0..10).rev() {
        for _ in 0..dp[n][i] {
            print!("{}", i);
        }
    }
    println!();
}

// aの方がおおきとき、true
fn cmp(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    let asum = a.iter().sum::<usize>();
    let bsum = b.iter().sum::<usize>();
    if bsum < asum {
        return true;
    }
    if asum < bsum {
        return false;
    }
    for i in (0..10).rev() {
        if b[i] < a[i] {
            return true;
        }
        if a[i] < b[i] {
            return false;
        }
    }
    // 同じ場合はtrue
    true
}
