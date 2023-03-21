//use std::io;
//use chrono::{DateTime, Local, NaiveTime, TimeZone};
use chrono::prelude::*;

use chrono_tz::Canada::Eastern;


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
    let dt = Utc::now();
    let ottawa_time = Eastern.from_utc_datetime(&dt.naive_utc());
    println!("Roman Clock: {}:{}:{}", roman_numeral(ottawa_time.hour()),roman_numeral(ottawa_time.minute()),roman_numeral(ottawa_time.second()));

}
