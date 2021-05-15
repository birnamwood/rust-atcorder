use proconio::input;

fn main() {
    input!{
        i: i32,
    }
 
    let a = (i -1).to_string();
    if a.len() == 4 {
        let b: i32 = a[0..2].parse().unwrap();
        println!("{}", b + 1);
    } else if a <= 0.to_string() {
        println!("{}", 1);
    } else {
        let b: i32 = a[0..1].parse().unwrap();
        println!("{}", b + 1);
    }
}