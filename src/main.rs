
use std::io;
enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}
fn main() {
    let exit_text = loop {
        println!("Type in 'c' for Celsius to Fahrenheit\nType in 'f' for Fahrenheit to Celsius\nType in 'e' for Exit");
        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Ok(_) => println!("Your input was: {}", selection),
            Err(error) => println!("error: {}", error),
        }
        let selection: char = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if selection == 'e' {
            break "Bye!"
        } else if selection == 'f' || selection == 'c' {
            loop {
                println!("Type in the number");
                let mut number = String::new();
                match io::stdin().read_line(&mut number) {
                    Ok(_) => println!("Your input was: {}", number),
                    Err(error) => println!("error: {}", error),
                }
                let number: f64 = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if selection == 'f' {
                    println!("{} Fahrenheit => {} Celsius", number, convert_temperature(&Temperature::Fahrenheit(number)));
                }
                if selection == 'c' {
                    println!("{} Celsius => {} Fahrenheit", number, convert_temperature(&Temperature::Celsius(number)));
                }
                break;
            }
        } else {
            continue;
        }
    };
    println!("{}",exit_text);
}

fn convert_temperature(temp: &Temperature) -> f64 {
    match temp {
        &Temperature::Fahrenheit(degrees) => (degrees - 32.) / (9./5.),
        &Temperature::Celsius(degrees) => (degrees * (9./5.)) + 32.,
    }
}
