use std::io;

fn main() {
    println!("Please enter the temperature in fahrenheit:");

    let mut fahrenheit_temperature = String::new();

    io::stdin()
        .read_line(&mut fahrenheit_temperature)
        .expect("Please type a number.");
    
    let fahrenheit_temperature: f64 = match fahrenheit_temperature.trim().parse() {
        Ok(temperature) => temperature,
        Err(_) => 0.0,
    };

    let celsius = convert_fahrenheit_to_celsius(fahrenheit_temperature);
    print_coversion_details(fahrenheit_temperature, celsius)
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    celsius
}

fn print_coversion_details(fahrenheit: f64, celsius: f64) {
    println!("{fahrenheit}°F is: {:.3}°C", celsius);
}