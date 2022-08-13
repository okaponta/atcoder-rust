use proconio::input;

fn main() {
    input! {
        l:usize,
        r:usize,
    }
    let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let mut ans: String = "".to_string();
    for i in l..r {
        ans.push(atcoder[i]);
    }
    println!("{}", ans);
}
