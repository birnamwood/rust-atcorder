use proconio::input;

fn main() {
    input! {
        a: String,
        s: String,
        c: String,
    }
    let name1 = a.chars().next().unwrap();
    let name2 = s.chars().next().unwrap();
    let name3 = c.chars().next().unwrap();

    println!("{}{}{}", name1, name2, name3);
}
