fn main() {
    let mut s = String::from("hi");

    let x = &s;
    let r = &mut s;

    println!("{x}");
    r.push_str("!");
}
