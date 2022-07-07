use std::io;

fn main() {
    println!("What's the temperature?");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");
    let temp: f32 = temp.trim().parse().expect("Failed to parse number to float");

    println!("Which unit is your temperature (Cel/Fahr)?");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read input");
    let unit = unit.trim();

    if unit == "Cel" {
        println!("{} Celsius is {} Fahrenheit", temp, cel_to_fahr(temp));
    } else {
        println!("{} Fahrenheit is {} Celsius", temp, fahr_to_cel(temp));
    }
}

fn cel_to_fahr(temp_cel: f32) -> f32 {
    temp_cel * 1.8 + 32.0
}

fn fahr_to_cel(temp_fahr: f32) -> f32 {
    (temp_fahr - 32.0) / 1.8
}
