use std;

fn main() {
    println!("Computing fibonacci N-th number");

    let index: u32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    println!("index is {index}");
    
    let result = better_fib(index);

    println!("Fibonacci sequence value at this index is {result}");
}

fn better_fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 1..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    
    b
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    }

    fib(n-1) + fib(n-2)
}

