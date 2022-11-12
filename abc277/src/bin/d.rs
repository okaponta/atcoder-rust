use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
    }
    a.sort();
    let mut sum = a[0];
    let mut clus = vec![];
    let mut temp = a[0];
    for i in 1..n {
        sum += a[i];
        if a[i] <= a[i - 1] + 1 {
            temp += a[i];
        } else {
            clus.push(temp);
            temp = a[i];
        }
    }
    clus.push(temp);
    if a[0] == 0 && a[n - 1] == m - 1 {
        let last = clus.pop().unwrap();
        if clus.len() == 0 {
            clus.push(last);
        } else {
            clus[0] += last;
        }
    }
    println!("{}", sum - clus.into_iter().max().unwrap());
}
