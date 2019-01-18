use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scale = &args[1];
    let value: i16 = args[2].trim().parse()
        .expect("Please enter number!");

    if scale == "f" {
        fahrenheit(value);
    } else if scale == "c" {
        celsius(value);
    } else if scale == "k" {
        kelvin(value);
    } else {
        println!("The first argument given must be 'c' or 'f' to specify the current scale.");
    };
}

fn fahrenheit(value: i16) {
    println!("Converting {}° fahrenheit.", value);

    let celsius_temp = ((((f64::from(value) - 32.0) * (5.0 / 9.0)) * 100.0).round()) / 100.0;
    let kelvin_temp = (((celsius_temp + 273.15) * 100.0).round()) / 100.0;

    println!("{}° fahrenheit is equal to {}° celsius.", value, celsius_temp);
    println!("{}° fahrenheit is equal to {}° kelvin.", value, kelvin_temp);
}

fn celsius(value: i16) {
    println!("Converting {}° celsius.", value);

    let fahrenheit_temp = ((((f64::from(value) * (9.0 / 5.0)) + 32.0) * 100.0).round()) / 100.0;
    let kelvin_temp = (((f64::from(value) + 273.15) * 100.0).round()) / 100.0;

    println!("{}° celsius is equal to {}° fahrenheit.", value, fahrenheit_temp);
    println!("{}° celsius is equal to {}° kelvin.", value, kelvin_temp);
}

fn kelvin(value: i16) {
    println!("Converting {}° kelvin.", value);

    let celsius_temp = (((f64::from(value) - 273.15) * 100.0).round()) / 100.0;
    let fahrenheit_temp = ((((celsius_temp * (9.0 / 5.0)) + 32.0) * 100.0).round()) / 100.0;

    println!("{}° kelvin is equal to {}° fahrenheit.", value, fahrenheit_temp);
    println!("{}° kelvin is equal to {}° celsius.", value, celsius_temp);
}
