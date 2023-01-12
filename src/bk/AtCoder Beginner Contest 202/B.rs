use proconio::input;
 
fn main() {
    input!{
        a: String,
    }
    let mut b: String ="".to_string();
    let six: &str = "6";
    let nine: &str = "9";
    for i in a.as_str().chars() {
        if i.to_string() == "6" {
            b = b + nine;
        } else if i.to_string() == "9" {
            b = b + six;
        } else {
            b = b + &i.to_string();
        }
    }
     println!("{}", b.chars().rev().collect::<String>());
}