use proconio::input;

fn main() {
    input! {
       n:usize,
    }
    if n % 2 != 0 {
        println!();
        return;
    }
    dfs(vec!['('; n], 0, 0, n / 2);
}

fn dfs(cur: Vec<char>, left: usize, right: usize, max: usize) {
    if left > max || right > max || left < right {
        return;
    }
    if left == max && right == max {
        println!("{}", cur.iter().collect::<String>());
        return;
    }
    let target = left + right;
    let mut for_left = cur.clone();
    for_left[target] = '(';
    dfs(for_left, left + 1, right, max);
    let mut for_right = cur.clone();
    for_right[target] = ')';
    dfs(for_right, left, right + 1, max);
}
