use std::i32;
use std::io;

const PRECISION: usize = 2;

fn convert_temperature(temp: f64, unit: Option<char>) {
    let unit = unit.expect("Not a valid unit");
    let kelvin = match unit {
        'k' => temp,
        'c' => temp + 273.15,
        'r' => temp / 1.8,
        'f' => (temp - 32.0) / 1.8 + 273.15,
        _ => return (),
    };
    let celcius = match unit {
        'k' => temp - 273.15,
        'c' => temp,
        'r' => temp * 1.8 + 491.67,
        'f' => (temp - 32.0) / 1.8,
        _ => return (),
    };
    let rankine = match unit {
        'k' => temp * 1.8,
        'c' => (temp - 491.67) / 1.8,
        'r' => temp,
        'f' => temp + 459.67,
        _ => return (),
    };
    let farenheit = match unit {
        'k' => (temp - 273.15) * 1.8 + 32.0,
        'c' => temp * 1.8 + 32.0,
        'r' => temp - 459.67,
        'f' => temp,
        _ => return (),
    };
    println!("{:>16.1$}°C", celcius, PRECISION);
    println!("{:>16.1$}°F", farenheit, PRECISION);
    println!("{:>16.1$}°K", kelvin, PRECISION);
    println!("{:>16.1$}°R", rankine, PRECISION);
}

fn main() {
    loop {
        println!("==================");

        println!("Enter Temperature:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let unit = if input.contains("k") || input.contains("K") {
            Some('k')
        } else if input.contains("c") || input.contains("C") {
            Some('c')
        } else if input.contains("r") || input.contains("R") {
            Some('r')
        } else if input.contains("f") || input.contains("F") {
            Some('f')
        } else {
            {
                println!("Use valid units!");
                continue;
            }
        };

        println!("NUMBER MAKER:");
        number_maker(input.clone());

        let temp: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Must be a number!");
                continue;
            }
        };

        println!("------------------");

        convert_temperature(temp, unit);
    }
}

fn number_maker(input: String) {
    let mut decimals: u8 = 0;
    let mut num: f64 = if input.contains("-") { -0.0 } else { 0.0 };
    for i in 0..input.len() - 1 {
        println!("Loop {i}: {num}");
        if input.chars().nth(i).expect("Char error").is_digit(10) == true {
            let digit = input.chars().nth(i).expect("Not a digit").to_digit(10);
            num = num * 10.0 + digit as f64;
            match decimals {
                0 => continue,
                _ => decimals += 1,
            }
            continue;
        } else if input.chars().nth(i) == Some('.') && decimals == 0 {
            decimals = 1;
            continue;
        } else {
            continue;
        }
    }
    num = num / 10_u64.pow(decimals.into()).into();
    println!("{num}");
}
