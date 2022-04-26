use std::io;

struct Calculator;
impl Calculator {
    fn read_data(&self, message: &str, error_message: &str) -> String {
        let mut data = String::new();    
        println!("{}", message);
        io::stdin()
            .read_line(&mut data)
            .expect(error_message);
        
        data.trim().to_string()
    }

    fn calculate(&self, number_1: &String, number_2: &String, operator: &String) -> i32 {
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
}

fn main() {
    let calculator = Calculator;
    let number_1: String = calculator.read_data("Elso szam:", "Hiba tortent a beolvasas soran!");
    let number_2: String = calculator.read_data("Masodik szam:", "Hiba tortent a beolvasas soran!");
    let operator: String = calculator.read_data("Operator ('+', '-', '*', '/'):", "Hiba tortent a beolvasas soran!");

    let result: i32 = calculator.calculate(&number_1, &number_2, &operator);
    println!("{} {} {} = {}", number_1, operator, number_2, result);
}
