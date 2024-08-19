use std::{io::stdin, process};

fn main() {
    loop {
        println!("-------------------------------------");
        println!("Please select the conversion type:");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        println!("q: Quit");

        let mut conversion_type = String::new();
        stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read selection, please try again");

        let conversion_type: i8 = match conversion_type.trim() {
            "1" => {
                println!("Your selected 1 Fahrenheit to Celsius conversion");
                1
            }
            "2" => {
                println!("Your selected 2 Celsius to Fahrenheit conversion");
                2
            }
            "q" => {
                println!("Your selected q to quit the program.");
                process::exit(1);
            }
            &_ => {
                println!("Please select 1 or 2 from the list");
                continue;
            }
        };

        println!("Please enter the temperature: ");
        let mut temp = String::new();
        stdin()
            .read_line(&mut temp)
            .expect("Failed to read selection, please try again");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature");
                continue;
            }
        };

        let converted_temp = if conversion_type == 1 {
            (temp - 32.0) / 1.8
        } else {
            temp * 1.8 + 32.0
        };

        println!(
            "The converted temperature in {} is: {converted_temp}",
            if conversion_type == 1 {
                "Fahrenheit"
            } else {
                "Celsius"
            }
        )
    }
}
