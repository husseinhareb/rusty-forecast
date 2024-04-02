use std::env;

mod instance;
mod city;
fn help() {
    println!("            -h      for help
            -c      to change city
            -d      to load default city
            -t      to see more information of todays weather
            -w      to see the weeks weather
    ")
}
fn main() {
    if let Err(err) = instance::weather_now() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = city::create_config() {
        eprintln!("Error: {}", err);
    }
    for argument in env::args() {
        if argument == "-h"{
            help();
        }
        if argument == "-d" {
            if let Err(err) = city::default_city() {
                eprintln!("Error: {}",err);
            }
        }
    }
}
