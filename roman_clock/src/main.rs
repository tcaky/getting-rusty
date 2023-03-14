//use std::io;
//use chrono::{DateTime, Local, NaiveTime, TimeZone};
use chrono::prelude::*;

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



fn main() {
    
    // let now = Local::now();
    // println!("Time: {}", now);
    // let hour = now.hour();

    //let dt = Utc::now().with_timezone(&FixedOffset::east_opt(-5*3600));

    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    
    let time_without_zone = NaiveDateTime::from_timestamp(timestamp, 0);
    println!("time_without_zone: {}", time_without_zone);
    // 2009-02-13 23:31:30
    let zoned: DateTime<FixedOffset> = DateTime::from_utc(time_without_zone, FixedOffset::east(-5 * 3600));
    println!("zoned: {}", zoned);
    // 2009-02-14 07:31:30 +08:00
    //let dt = Local::now();
    // println!("Current Hour  : {}", dt.hour());
    // println!("Current Minute: {}", dt.minute());
    // println!("Current Second: {}", dt.second());

    // Prompt the user to enter a number
    //println!("Enter a number to convert to Roman numerals:");
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).unwrap();
    //let num: u32 = input.trim().parse().unwrap();
    // let num: u32 = 14;
    // let result = roman_numeral(num);
    roman_numeral(8);
    //println!("Roman numeral: {}", result);
    //println!("Roman Clock: {}:{}:{}", roman_numeral(dt.hour()),roman_numeral(dt.minute()),roman_numeral(dt.second()));

}
