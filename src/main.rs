use std::io;

fn main() {
    let mut buffer: String = String::new();

    println!("введіть перше число");
    io::stdin().read_line(&mut buffer);
    let num1: f64 = buffer.trim().parse().expect("помилка парсингу");
    buffer.clear();

    println!("введіть друге число");
    io::stdin().read_line(&mut buffer);
    let num2: f64 = buffer.trim().parse().expect("помилка парсингу");
    buffer.clear();

    input(num1, num2);
}
fn calc(symbol: char, value1: f64, value2: f64) {
    if symbol == '+' {
        println!("{} + {} = {}", value1, value2, value1 + value2);
    } else if symbol == '-' {
        println!("{} - {} = {}", value1, value2, value1 - value2);
    } else if symbol == '*' {
        println!("{} * {} = {}", value1, value2, value1 * value2);
    } else if symbol == '/' {
        println!("{} / {} = {}", value1, value2, value1 / value2);
    } else if symbol == 'a' {
        calc('+', value1, value2);
        calc('-', value1, value2);
        calc('*', value1, value2);
        calc('/', value1, value2);
    } else {
        println!("ПОМИЛКА: введіть правильний математичний знак (+, -, *, /, a)");
        input(value1, value2);
    }
}
fn input(num1: f64, num2: f64) {
    let mut buffer: String = String::new();

    println!("введіть математичний знак (+, -, *, /, a)");
    io::stdin().read_line(&mut buffer);
    let symbol: char = buffer.trim().parse().expect("помилка парсингу");
    buffer.clear();

    println!("");
    println!("результат: ");

    calc(symbol, num1, num2);
}
