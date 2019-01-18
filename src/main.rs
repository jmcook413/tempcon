use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scale = &args[1];
    let value: i16 = args[2].trim().parse()
        .expect("Please enter number!");

    if scale == "f" {
        f_to_c(value);
    } else if scale == "c" {
        c_to_f(value);
    } else {
        println!("The first argument given must be 'c' or 'f' to specify the current scale.");
    };
}

fn f_to_c(value: i16) {
    println!("Converting {}° fahrenheit to celsius.", value);

    let converted_value = ((f64::from(value) - 32.0) * (5.0 / 9.0)).round();

    println!("{}° farenheit is equal to {}° celsius.", value, converted_value);
}

fn c_to_f(value: i16) {
    println!("Converting {}° celsius to fahrenheit.", value);

    let converted_value = ((f64::from(value) * (9.0 / 5.0)) + 32.0).round();

    println!("{}° celsius is equal to {}° farenheit .", value, converted_value);
}
