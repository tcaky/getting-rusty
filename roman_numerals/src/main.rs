use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number to convert to Roman numerals:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: u32 = input.trim().parse().unwrap();

    let result = int_to_roman(num);
    println!("Roman numeral type 2: {}", result);
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