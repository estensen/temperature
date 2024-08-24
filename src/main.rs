use std::io;

fn main() {
    println!("Enter the temperature you want to convert");
    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read temperature input");
    let temp: f64 = temp_input
        .trim()
        .parse()
        .expect("Please add a valid number");

    println!(
        r#"Enter the conversion type:
1: Celsius to Fahrenheit
2: Celsius to Kelvin
3: Fahrenheit to Celsius
4: Fahrenheit to Kelvin
5: Kelvin to Celsius
6: Kelvin to Fahrenheit"#
    );
    let mut conversion_input = String::new();
    io::stdin()
        .read_line(&mut conversion_input)
        .expect("Failed to read conversion input");
    let conversion_type: u32 = conversion_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    match conversion_type {
        1 => println!("{}°C is {}°F", temp, celsius_to_fahrenheit(temp)),
        2 => println!("{}°C is {}°K", temp, celsius_to_kelvin(temp)),
        3 => println!("{}°F is {}°C", temp, fahrenheit_to_celsius(temp)),
        4 => println!("{}°F is {}°K", temp, fahrenheit_to_kelvin(temp)),
        5 => println!("{}°K is {}°C", temp, kelvin_to_celsius(temp)),
        6 => println!("{}°K is {}°F", temp, kelvin_to_fahrenheit(temp)),
        _ => println!("Invalid conversion type selected"),
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    celsius_to_kelvin(fahrenheit_to_celsius(fahrenheit))
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let celsius = 0.0;
        let fahrenheit = celsius_to_fahrenheit(celsius);
        assert_eq!(fahrenheit, 32.0);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let celsius = 0.0;
        let kelvin = celsius_to_kelvin(celsius);
        assert_eq!(kelvin, 273.15);
    }
}
