use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let c= a.to_string();
    let d= b.to_string();
    if c.chars().count() == 1 && d.chars().count() == 1 {
        println!("{}", a * b)
    }else {
        println!("{}", -1);
    }
}