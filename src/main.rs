use std::env;

mod instance;
mod city;

fn help() {
    println!("Usage:
        -h      Display help
        -c      Change city
        -d      Load default city
        -t      See more information for today's weather
        -w      See the week's weather");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // If no arguments are provided
        instance::weather_now();
        return;
    }

    // Match each command-line argument
    for arg in &args[1..] {
        match arg.as_str() {
            "-h" => help(),
            "-c" => {

            }
            "-d" => {
                if let Err(err) = city::read_city_name() {
                    eprintln!("Error: {}", err);
                }
            }
            "-t" => {
                if let Err(err) = instance::weather_now() {
                    eprintln!("Error: {}", err);
                }
            }
            "-w" => {

            }
            _ => {
                eprintln!("Invalid argument: {}", arg);
                help();
            }
        }
    }
}
