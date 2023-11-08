use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        c:usize,
        s:Chars,
    }
    let mut min = vec![];
    for i in 0..n {
        if s[i] == 'o' {
            if min.len() == 0 {
                min.push(i);
            } else if min[min.len() - 1] + c < i {
                min.push(i);
            }
        }
    }
    if min.len() != k {
        return;
    }
    let mut max = vec![];
    for i in (0..n).rev() {
        if s[i] == 'o' {
            if max.len() == 0 {
                max.push(i);
            } else if max[max.len() - 1].saturating_sub(c) > i {
                max.push(i);
            }
        }
    }
    max.reverse();
    for i in 0..k {
        if min[i] == max[i] {
            println!("{}", min[i] + 1);
        }
    }
}
