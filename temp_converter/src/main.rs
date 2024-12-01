use std::io;

fn main() {
  println!("Temperature Converter");

  let temperature = get_temperature();
  let unit = get_unit();

  let result = if unit == "F" {
    fahrenheit_to_celsius(temperature)
  } else {
    celsius_to_fahrenheit(temperature)
  };

  let unit = if unit == "F" { "C" } else { "F" };

  println!("The temperature is {result} degrees {unit}");
}

fn get_temperature() -> f64 {
  println!("Enter the temperature: ");

  let input: f64 = match get_input().trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("ERROR: Please enter a number");
      return get_temperature();
    },
  };

  input
}

fn get_unit() -> String {
  println!("Enter the unit of temperature (F or C): ");

  let input = get_input().trim()
    .to_string()
    .to_uppercase();

  if input != "F" && input != "C" {
    println!("ERROR: Please enter 'F' or 'C' for Fahrenheit or Celsius");
    return get_unit();
  }

  input
}

fn get_input() -> String {
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("FATAL: Failed to read line");

  input
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
  celsius * 9.0 / 5.0 + 32.0
}