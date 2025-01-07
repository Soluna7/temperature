use regex::Regex;
use std::io;

const PRECISION: usize = 2;
fn main() {
    loop {
        println!("===================");

        println!("Enter Temperature:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let unit = if input.contains("k") || input.contains("K") {
            Some('K')
        } else if input.contains("c") || input.contains("C") {
            Some('C')
        } else if input.contains("r") || input.contains("R") {
            Some('R')
        } else if input.contains("f") || input.contains("F") {
            Some('F')
        } else {
            {
                println!("Use valid units!");
                continue;
            }
        };

        let sign: f64 = if input.contains('-') { -1.0 } else { 1.0 };

        let pattern = Regex::new(r"[^\d.]").unwrap();
        let input = pattern.replace_all(&input, "");

        let temp: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Must be a number!");
                continue;
            }
        };

        let temp = temp * sign;
        let unit_symbol = unit.unwrap();

        println!("-------------------");

        println!("{temp}°{unit_symbol} =");

        println!("* - - - - - - - - *");

        convert_temperature(temp, unit);
    }
}

fn convert_temperature(temp: f64, unit: Option<char>) {
    let unit = unit.expect("Not a valid unit");
    let kelvin = match unit {
        'K' => temp,
        'C' => temp + 273.15,
        'R' => temp / 1.8,
        'F' => (temp - 32.0) / 1.8 + 273.15,
        _ => return (),
    };
    let celcius = match unit {
        'K' => temp - 273.15,
        'C' => temp,
        'R' => temp * 1.8 + 491.67,
        'F' => (temp - 32.0) / 1.8,
        _ => return (),
    };
    let rankine = match unit {
        'K' => temp * 1.8,
        'C' => (temp - 491.67) / 1.8,
        'R' => temp,
        'F' => temp + 459.67,
        _ => return (),
    };
    let farenheit = match unit {
        'K' => (temp - 273.15) * 1.8 + 32.0,
        'C' => temp * 1.8 + 32.0,
        'R' => temp - 459.67,
        'F' => temp,
        _ => return (),
    };
    if kelvin < 0.0 {
        println!("INVALID TEMPERATURE");
        println!("* - - - - - - - - *");
    }
    println!("{:>17.1$}°C", celcius, PRECISION);
    println!("{:>17.1$}°F", farenheit, PRECISION);
    println!("{:>17.1$}°K", kelvin, PRECISION);
    println!("{:>17.1$}°R", rankine, PRECISION);
}
