use proconio::input;
 
fn main() {
    input!{
        array: [i32; 3],
    }
    let mut a = array;
    for i in 0..a.len() - 1 {
        for j in (i + 1..a.len()).rev() {
            if a[j - 1] > a[j] {
                a.swap(j, j - 1);
            }
        }
    }
    if a[2] - a[1] == a[1] - a[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}