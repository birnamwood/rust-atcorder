use proconio::input;

fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
    }
    let mut array = [a,b,c];
    array.sort();


    println!("{}", array[2] + array[1]);
}