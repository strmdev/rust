use std::io;

struct Calculator<'a> {
    number_1: &'a str,
    number_2: &'a str,
    operator: &'a str,
}

impl<'a> Calculator<'a> {
    pub fn new() -> Self {
        Self {
            number_1: "",
            number_2: "",
            operator: "",
        }
    }

    pub fn read_data(&self, message: &str, error_message: &str) -> String {
        let mut data: String = String::new();  
          
        println!("{}", message);
        io::stdin()
            .read_line(&mut data)
            .expect(error_message);
        
        data.trim()
            .to_string()
    }

    pub fn calculate(&mut self, number_1: &'a str, number_2: &'a str, operator: &'a str) -> f32 {
        let mut result: f32 = 0.0;
        
        self.number_1 = number_1;
        self.number_2 = number_2;
        self.operator = operator;

        let converted_number_1: f32 = self.number_1.trim()
                                                   .parse::<f32>()
                                                   .unwrap();
        let converted_number_2: f32 = self.number_2.trim()
                                                   .parse::<f32>()
                                                   .unwrap();
    
        match self.operator.trim() {
            "+" => result = converted_number_1 + converted_number_2,
            "-" => result = converted_number_1 - converted_number_2,
            "*" => result = converted_number_1 * converted_number_2,
            "/" => result = converted_number_1 / converted_number_2,
            _   => println!("Az operator csak '+', '-', '*', '/' lehet!"),
        }
    
        result
    }
}

fn main() {
    let mut calculator = Calculator::new();
    
    let number_1: String = calculator.read_data("Elso szam:", "Hiba tortent az elso szam beolvasasa soran!");
    let number_2: String = calculator.read_data("Masodik szam:", "Hiba tortent a masodik szam beolvasasa soran!");
    let operator: String = calculator.read_data("Operator ('+', '-', '*', '/'):", "Hiba tortent az operator beolvasasa soran!");

    let result: f32 = calculator.calculate(&number_1, &number_2, &operator);
    println!("{} {} {} = {}", number_1, operator, number_2, result);
}