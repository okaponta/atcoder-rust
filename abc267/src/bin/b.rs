use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut pin = vec![false; 10];
    for i in 0..10 {
        if s[i] == '1' {
            pin[i] = true;
        }
    }
    if pin[0] {
        println!("No");
        return;
    }
    let mut row = vec![];
    row.push(pin[6]);
    row.push(pin[3]);
    row.push(pin[1] || pin[7]);
    row.push(pin[4]);
    row.push(pin[2] || pin[8]);
    row.push(pin[5]);
    row.push(pin[9]);
    let mut flag1 = false;
    let mut flag2 = false;
    for b in row {
        if b {
            flag1 = true;
            if flag2 {
                println!("Yes");
                return;
            }
        } else {
            if flag1 {
                flag2 = true;
            }
        }
    }
    println!("No");
}
