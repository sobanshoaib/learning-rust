use std::io;




fn celsius_to_farenheit(value: f64) -> f64 {
    let farenheit_value = (value * (9.0/5.0)) + 32.0;
    farenheit_value
}

fn farenheit_to_celsius(value: f64) -> f64 {
    let celcius_value = (value-32.0) * (5.0/9.0);
    celcius_value
}

fn celsius_to_kelvin(value: f64) -> f64 {
    let kelvin_value = value + 273.15;
    kelvin_value
}




fn main() {
    
    
    let valid_choices = ["1","2","3"];

    println!("Which calculation do you want to do?");
    println!("1 (C->F)");
    println!("2 (F->C)");
    println!("3 (C->K)");
    
    
    let mut conversion_number = String::new(); //empty string

    while !valid_choices.contains(&conversion_number.trim()) {  
        conversion_number.clear();  
        io::stdin()
        .read_line(&mut conversion_number)
        .expect("Failed to read line.");
    }

    let conversion_number: i32 = conversion_number
    .trim()
    .parse() //is the thing that converts string to int
    .expect("Failed to convert to number");

    println!("The number you entered is: {}", conversion_number);

    println!("Enter what number you want to convert:");

    let mut input_number = String::new();
    io::stdin()
    .read_line(&mut input_number)
    .expect("Failed to read line");
    
    let input_number: f64 = input_number
    .trim()
    .parse()
    .expect("Failed to convert to number");
    println!("The number you want to conver is: {}", input_number);

    if conversion_number == 1 {
        let x = celsius_to_farenheit(input_number);
        println!("{} to farenheit is: {}", input_number, x)
    } else if conversion_number == 2 {
        let y = farenheit_to_celsius(input_number);
        println!("{} to farenheit is: {}", input_number, y)
    } else {
        let z = celsius_to_kelvin(input_number);
        println!("{} to farenheit is: {}", input_number, z)
    }


}
