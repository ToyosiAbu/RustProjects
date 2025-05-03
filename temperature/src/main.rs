use std::io;

fn main() {
    println!("Good Morning, I am the Weatherman!");    

    loop {
        println!("Please enter your temperature in Celsius.");

        let mut celsius_start = String::new();

        io::stdin().read_line(&mut celsius_start)
            .expect("Failed to read line");

        let celsius_start: f64 = match celsius_start.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You inputted {}C", celsius_start);

        let mut fahrenheit = ((celsius_start * 9.0) / 5.0) + 32.0;
        println!("The temperature in Farenheit is {}F", fahrenheit);
    }
}
