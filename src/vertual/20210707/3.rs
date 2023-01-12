use proconio::{input, marker::Chars};
 
fn main() {
    input!{
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    // let cs: Vec<char> = s.chars().collect();

    println!("{:?}", a);
}