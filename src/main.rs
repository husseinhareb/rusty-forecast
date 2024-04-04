use std::env;

mod instance;
mod city;
mod condition_icons;


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
        let _ = instance::weather_now(); 
        let _ = city::create_config(); 
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
                // Read the next argument as the city name
                if let Some(city_name) = iter.next().map(|s| s.to_owned()) {
                    let _ = city::write_city_name(&city_name);
                } else {
                    eprintln!("City name not provided for the -c flag.");
                    help();
                    return;
                }
            }
            "-d" => {
                if let Err(err) = city::write_def_city() {
                    eprintln!("Error: {}", err);
                }
            }
            "-t" => {
                if let Err(err) = instance::weather_now() {
                    eprintln!("Error: {}", err);
                }
            }
            "-w" => {
                // Implement handling for the "-w" flag if needed
            }
            _ => {
                eprintln!("Invalid argument: {}", arg);
                help();
                return;
            }
        }
    }
}