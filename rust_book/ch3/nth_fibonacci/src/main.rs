use std::io;

fn main() {
    // Prompt the user to enter a number
    loop {
        println!("Enter a number calculate Fibonacci value at that position:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let fib_position: u32 = input.trim().parse().unwrap();
    
        let fib_output: u64 = nth_fibonacci(fib_position);
        println!("Position {} in the Fibonacci sequence has a value of: {}", fib_position, fib_output);

    }
}


// nth_fibonacci
fn nth_fibonacci(nth_pos: u32) -> u64{
    if nth_pos == 0 {return 0;}

    let mut prev = 0 as u64;
    let mut current = 1 as u64;
    for _ in 1..nth_pos{
        let temp = prev;
        prev = current;
        current = temp + current;

    }
    current
}