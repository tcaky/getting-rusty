use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number to convert to Roman numerals:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: u32 = input.trim().parse().unwrap();

    let result = roman_numeral(num);
    println!("Roman numeral type 1: {}", result);

    let result = int_to_roman(num);
    println!("Roman numeral type 2: {}", result);
}


fn roman_numeral(num: u32) -> String {
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
        while remaining >= value {
            result.push(symbol);
            remaining -= value;
        }

        // Check for subtractive notation
        if remaining > 0 {
            let next_value = symbols.iter().find(|&&(_, v)| v <= remaining);
            if let Some(&(next_symbol, next_value)) = next_value {
                let difference = value - next_value;
                if difference <= remaining {
                    result.push(next_symbol);
                    result.push(symbol);
                    remaining -= difference;
                }
            }
        }
    }
    result
}


fn int_to_roman(number: u32) -> String {
    let mut result = String::new();

    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut remaining = number;
    for (value, numeral) in values.iter() {
        while remaining >= *value {
            result.push_str(numeral);
            remaining -= value;
        }
    }

    result
}