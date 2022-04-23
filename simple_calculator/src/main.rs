use std::io;

fn main() {
    let mut number_1 = String::new();    
    println!("Elso szam:");
    io::stdin()
        .read_line(&mut number_1)
        .expect("Hiba tortent az elso szam beolvasasa soran!");
    
    let mut number_2 = String::new();
    println!("Masodik szam:");
    io::stdin()
        .read_line(&mut number_2)
        .expect("Hiba tortent a masodik szam beolvasasa soran!");

    let mut operator = String::new();
    println!("Operator:");
    io::stdin()
        .read_line(&mut operator)
        .expect("Hiba tortent az operator beolvasasa soran!");

    let mut result: i32 = 0;
    match operator.trim() {
        "+" => result = number_1.trim().parse::<i32>().unwrap() + number_2.trim().parse::<i32>().unwrap(),
        "-" => result = number_1.trim().parse::<i32>().unwrap() - number_2.trim().parse::<i32>().unwrap(),
        "*" => result = number_1.trim().parse::<i32>().unwrap() * number_2.trim().parse::<i32>().unwrap(),
        "/" => result = number_1.trim().parse::<i32>().unwrap() / number_2.trim().parse::<i32>().unwrap(),
        _   => println!("Az operator csak '+', '-', '*', '/' lehet!"),
    }
    println!("Result: {}", result);
}
