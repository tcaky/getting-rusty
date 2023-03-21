use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number to convert to Roman numerals:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: u32 = input.trim().parse().unwrap();

    // Define Roman numeral symbols and their values
    let symbols = vec![
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1),
    ];

    // Build Roman numeral string
    let mut result = String::new();
    let mut remaining = num;
    for &(symbol, value) in symbols.iter() {
        dbg!(&symbol);
        while remaining >= value {
            println!("PUSHING SYMBOL: {}", symbol);
            result.push(symbol);
            remaining -= value;
        }

        // Check for subtractive notation
        if remaining > 0 {
            dbg!(&remaining);
            let next_value = symbols.iter().find(|&&(_, v)| v <= remaining);
            dbg!(&next_value);
            if let Some(&(next_symbol, next_value)) = next_value {
                let difference = value - next_value;
                dbg!(&difference);
                println!(" ");
                if difference <= remaining {
                    result.push(next_symbol);
                    result.push(symbol);
                    remaining -= difference;
                }
            }
        }
    }

    // Output Roman numeral string
    println!("Roman numeral: {}", result);
}
