use std::io;

fn main() {
    const APP_NAME: &str = "Calculator";
    const VERSION: f32 = 0.1;

    let num1: f64;
    let num2: f64;
    let operator: String;
    let result: f64;

    welcome(APP_NAME, VERSION);
    num1 = get_num("Enter first number: ");
    num2 = get_num("Enter second number: ");
    operator = get_operator();
    result = get_result(num1, num2, operator.as_str());

    println!("{} {} {}: {}", num1, operator.trim(), num2, result);
}

fn welcome(app_name: &str, version: f32) {
    println!("Welcome to {} version {}", app_name, version);
}

fn get_num(message: &str) -> f64 {
    let mut num_text = String::new();

    println!("{}", message);
    io::stdin()
        .read_line(&mut num_text)
        .expect("Read line failed.");

    let num: f64 = match num_text.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return get_num(message); // Recursively ask for input again
        }
    };

    return num;
}

fn get_operator() -> String {
    let mut operator = String::new();
    let operator_str: &str;

    println!("Choose an operator (+,-,*,/): ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Read line failed.");

    operator_str = operator.as_str().trim();

    if operator_str == "+" || operator_str == "-" || operator_str == "*" || operator_str == "/" {
        return operator;
    }

    return get_operator();
}

fn get_result(num1: f64, num2: f64, operator: &str) -> f64 {
    let operator: &str = operator.trim();
    let result: f64;

    match operator {
        "+" => result = num1 + num2,
        "-" => result = num1 - num2,
        "*" => result = num1 * num2,
        "/" => {
            if num2 == 0.0 {
                result = 0.0;
            } else {
                result = num1 / num2
            };
        }
        _ => {
            println!("Invalid operator");
            result = 0.0;
        }
    }

    return result;
}
