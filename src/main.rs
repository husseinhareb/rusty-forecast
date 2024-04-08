use std::env;

mod config;
mod weather;
mod city;
mod condition_icons;

fn help() {
    println!("Usage: rusty-forecast [options] | rusty-forecast");
    println!("Options:");   
    println!("-h               Display this help message");     
    println!("-c <city_name>   Change the city name");
    println!("-d               Set the default city according to timezone");
    println!("-t               Show more weather details of today");
    println!("-w               Show weather forecast");
    println!("-s               Show all configuration settings");
    println!("-u <unit>        Set the unit of temperature (Celsius or Fahrenheit)");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let _ = config::create_config(); 
        let _ = weather::weather_now(); 
        return;
    }

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
                if let Err(err) = weather::weather_details() {
                    eprintln!("Error: {}", err);
                }
            }
            "-w" => {
                if let Err(err) = weather::weather_forecast() {
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