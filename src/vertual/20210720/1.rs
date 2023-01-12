use proconio::input;

fn main() {
    input!{
        h: usize,
        a: [[i32; 2]; h],
    }
    let mut count: i32 = 0;

    for i in 0..a.len() {
        if a[i][0] == a[i][1] {
            count += 1;
        } else {
            count = 0;
        }
        if count == 3 {
            break;
        }
    }

    if count == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}