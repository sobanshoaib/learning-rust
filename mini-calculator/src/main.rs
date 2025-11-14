use std::io;




fn add_numbers(value_one: f64, value_two:f64) -> f64 {
    let result = value_one + value_two;
    result
}

fn subtract_numbers(value_one: f64, value_two:f64) -> f64 {
    let result = value_one - value_two;
    result
}

fn multiply_numbers(value_one: f64, value_two:f64) -> f64 {
    let result = value_one * value_two;
    result
}


fn divide_number(value_one: f64, value_two:f64) -> f64 {
    let result = value_one / value_two;
    result
}

fn modulo_number(value_one: f64, value_two:f64) -> f64 {
    let result = value_one % value_two;
    result
}


fn main() {
    let mut first_number = String::new();
    println!("Enter number #1:");
    io::stdin()
    .read_line(&mut first_number)
    .expect("Failed to read input");
    let first_number: f64 = first_number
    .trim()
    .parse()
    .expect("Failed to convert to int");


    let mut operator = String::new();
    println!("Enter operator:");
    io::stdin()
    .read_line(&mut operator)
    .expect("Failed to read input");
    let operator = operator
    .trim();

    let mut second_number = String::new();
    println!("Enter number #2:");
    io::stdin()
    .read_line(&mut second_number)
    .expect("Failed to read input");
    let second_number: f64 = second_number
    .trim()
    .parse()
    .expect("Failed to convert to int");

    if operator == "+" {
        let result = add_numbers(first_number, second_number);
        println!("The result of {} {} {} is: {}", first_number, operator, second_number, result);
    } else if operator == "-" {
        let result = subtract_numbers(first_number, second_number);
        println!("The result of {} {} {} is: {}", first_number, operator, second_number, result);
    } else if operator == "*" {
        let result = multiply_numbers(first_number, second_number);
        println!("The result of {} {} {} is: {}", first_number, operator, second_number, result);
    } else if operator == "/" {
        let result = divide_number(first_number, second_number);
        println!("The result of {} {} {} is: {}", first_number, operator, second_number, result);
    } else if operator == "%" {
        let result = modulo_number(first_number, second_number);
        println!("The result of {} {} {} is: {}", first_number, operator, second_number, result);
    }
}
