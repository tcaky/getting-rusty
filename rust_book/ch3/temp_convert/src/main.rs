fn main() {
    let original_temp_in_c: f32 = 20 as f32;
    println!("Temp in {} C", original_temp_in_c);
    let temp_in_f = c_to_f(original_temp_in_c);
    println!("Converted to F: {}", temp_in_f);
    let temp_in_c = f_to_c(temp_in_f);
    println!("Converted to C: {}", temp_in_c);

}

// To convert temperatures in degrees Fahrenheit to Celsius, subtract 32 and multiply by .5556 (or 5/9).
fn f_to_c(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0/9.0)
}
// To convert temperatures in degrees Celsius to Fahrenheit, multiply by 1.8 (or 9/5) and add 32.
fn c_to_f(celsius: f32) -> f32 {
    celsius * (9.0/5.0) + 32.0
}