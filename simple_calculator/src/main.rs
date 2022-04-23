use std::io;

fn read_data(message: &str, error_message: &str) -> String {
    let mut data = String::new();    
    println!("{}", message);
    io::stdin()
        .read_line(&mut data)
        .expect(error_message);
    
    data
}

fn calculate(number_1: String, number_2: String, operator: String) -> i32 {
    let mut result: i32 = 0;

    match operator.trim() {
        "+" => result = number_1.trim().parse::<i32>().unwrap() + number_2.trim().parse::<i32>().unwrap(),
        "-" => result = number_1.trim().parse::<i32>().unwrap() - number_2.trim().parse::<i32>().unwrap(),
        "*" => result = number_1.trim().parse::<i32>().unwrap() * number_2.trim().parse::<i32>().unwrap(),
        "/" => result = number_1.trim().parse::<i32>().unwrap() / number_2.trim().parse::<i32>().unwrap(),
        _   => println!("Az operator csak '+', '-', '*', '/' lehet!"),
    }

    result
}

fn main() {
    let number_1 = read_data("Elso szam:", "Hiba tortent az elso szam beolvasasa soran!");
    let number_2 = read_data("Masodik szam:", "Hiba tortent a masodik szam beolvasasa soran!");
    let operator = read_data("Operator:", "Hiba tortent az operator beolvasasa soran!");

    let result   = calculate(number_1, number_2, operator);
    println!("Result: {}", result);
}
