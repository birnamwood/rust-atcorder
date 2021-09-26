use proconio::input;

fn main() {
    input!{
        n: i32,
    }
    let a:i32 = n % 10;

    match a {
        0 | 1 | 6 | 8 => println!("pon"),
        2 | 4 | 5 | 7 | 9 => println!("hon"),
        3  => println!("bon"),
        _ => println!("anything"), 
    }
}