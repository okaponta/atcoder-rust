use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        x:usize,
        w:[usize;n],
        k:[Usize1;q],
    }
    let s = w.iter().sum::<usize>();
    let mut map = vec![];
    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    let mut count = 0;
    loop {
        if left == n {
            // しゃくとり終了
            break;
        }
        if sum < x {
            count += ((x - sum) / s) * n;
            sum += ((x - sum) / s) * s;
        }
        while sum < x {
            sum += w[right];
            count += 1;
            right += 1;
            right %= n;
        }
        map.push((right, count));
        sum -= w[left];
        count -= 1;
        left += 1;
    }
    let mut used = vec![false; n];
    let mut path = vec![];
    let mut next = 0;
    loop {
        if used[next] {
            break;
        }
        path.push(next);
        used[next] = true;
        next = map[next].0;
    }
    let mut first = 0;
    for i in 0..n {
        if path[i] == next {
            first = i;
            break;
        }
    }
    let loop_size = path.len() - first;
    for mut ki in k {
        if ki <= first {
            println!("{}", map[path[ki]].1);
            continue;
        }
        ki -= first;
        ki %= loop_size;
        println!("{}", map[path[ki + first]].1)
    }
}
