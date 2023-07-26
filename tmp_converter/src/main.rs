use std::io;
fn main() {
    loop {
        println!("hey hey...");
        println!("it's Temperature Converter!");
        println!("Press 1 to convert from Celsius to Fahrenheit.");
        println!("Press 2 to convert from Fahrenheit to Celsius.");
        println!("Press 3 to exit.");
        println!("*************************************************");

        let mut menu_input = String::new();
        io::stdin()
            .read_line(&mut menu_input)
            .expect("Faild to read");

        match menu_input.trim().parse() {
            Ok(num) => match num {
                1 => celsius_to_fahrenheit(),
                2 => fahrenheit_to_celsius(),
                3 => {
                    println!("bye!");
                    break;
                }
                _ => continue,
            },
            Err(_) => continue,
        };
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter a Celsius temperature:");

    let mut temp_input = String::new();

    io::stdin()
        .read_line(&mut temp_input)
        .expect("Faild to read.");

    let temp_input = temp_input.trim().parse::<f64>().unwrap();
    let temp_converted: f64 = temp_input * 9.0 / 5.0 + 32.0;

    println!(
        "************************************************* \n
        {temp_input} C is {temp_converted} F. \n"
    );
}

fn fahrenheit_to_celsius() {
    println!("Enter a Fahrenheit temperature:");

    let mut temp_input = String::new();

    io::stdin()
        .read_line(&mut temp_input)
        .expect("Faild to read.");

    let temp_input = temp_input.trim().parse::<f64>().unwrap();
    let temp_converted: f64 = (temp_input - 32.0) * 5.0 / 9.0;

    println!(
        "************************************************* \n
        {temp_input} F is {temp_converted} C. \n"
    );
}
fn celsius_to_kelvin_converter() {
    println!("Enter a Celsius temperature:");

    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read.");

    let temp_input = temp_input.trim().parse::<f64>().unwrap();
    let temp_converted: f64 = celsius_to_kelvin(temp_input);

    println!(
        "************************************************* \n
        {} C is {} K. \n", temp_input, temp_converted
    );
}
