use std::io;

fn main() {
    println!("Enter celsius");

    let celsius: i8 = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to parse input");
        s.trim().parse().expect("Failed to parse celsius")
    };

    let farenheit = 9 / 5 * celsius + 32;
    
    println!("{farenheit} farenheit for {celsius} celsius");
}
