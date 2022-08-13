use proconio::{input, marker::Usize1};

fn main() {
    input! {
        l:Usize1,
        r:usize,
    }
    let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let mut ans: String = "".to_string();
    for i in l..r {
        ans.push(atcoder[i]);
    }
    println!("{}", ans);
}
