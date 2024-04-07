use std::env;

mod instance;
mod city;
mod condition_icons;
mod upcoming;
mod config;

fn help() {
    println!("Usage:
        -h      Display help
        -c      Change city
        -u      Change unit
        -d      Load default city
        -t      See more information for today's weather
        -s      Display settings
        -w      Display the week's weather");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let _ = config::create_config(); 
        let _ = instance::weather_now(); 
        return;
    }

    // Iterate through command-line arguments
    let mut iter = args.iter().skip(1); // Skip the first argument (program name)

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" => {
                help();
                return;
            }
            "-c" => {
                if let Some(city_name) = iter.next().map(|s| s.to_owned()) {
                    let _ = config::write_city_name(&city_name);
                } else {
                    eprintln!("City name not provided for the -c flag.");
                    help();
                    return;
                }
            }
            "-d" => {
                if let Err(err) = config::write_def_city() {
                    eprintln!("Error: {}", err);
                }
            }
            "-t" => {
                if let Err(err) = instance::weather_details() {
                    eprintln!("Error: {}", err);
                }
            }
            "-w" => {
                if let Err(err) = upcoming::weather_forecast() {
                    eprintln!("Error: {}", err);
                }
            }
            "-s" => {
                if let Err(err) = config::read_all_configs() {
                    eprintln!("Error: {}", err);
                }
            }
            "-u" => {
                if let Some(unit_value) = iter.next() {
                    if let Some(unit_char) = unit_value.chars().next() {
                        let unit = unit_char.to_ascii_uppercase();
                        if unit == 'C' || unit == 'F' {
                            let _ = config::write_unit(&unit);
                        } else {
                            eprintln!("Invalid unit value provided. Use 'C' or 'F'.");
                            return;
                        }
                    } else {
                        eprintln!("Invalid unit value provided. Use single characters 'C' or 'F'.");
                        return;
                    }
                }
                
            }
            
            _ => {
                eprintln!("Invalid argument: {}", arg);
                help();
                return;
            }
        }
    }
}