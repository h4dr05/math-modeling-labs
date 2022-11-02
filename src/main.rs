use std::io::{self, stdin, Write};
use textplots::{Chart, Plot, Shape};

const GRAVITY: f64 = 9.80665;

fn main() {
    std::env::set_var("LC_ALL", "ru_RU.UTF-8");

    print!("Введите вес мяча в кГ = ");
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading from STDIN");
    let mut mass = user_input
        .trim()
        .parse::<f64>()
        .expect("Error parsing input");

    mass /= GRAVITY; // рассчёт массы

    user_input.clear();
    print!("Введите сопротивление воздуха в кГ = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading from STDIN");
    let air_resistance_coefficient = user_input
        .trim()
        .parse::<f64>()
        .expect("Error parsing input");

    user_input.clear();
    print!("Введите начальную скорость мяча = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading from STDIN");
    let initial_speed = user_input
        .trim()
        .parse::<f64>()
        .expect("Error parsing input");

    println!("*****************************************");

    let time_air_resistance = f64::sqrt(mass / (GRAVITY * air_resistance_coefficient))
        * f64::atan(initial_speed * (f64::sqrt(air_resistance_coefficient / (mass * GRAVITY)))); //время
    let height_air_resistance = mass / air_resistance_coefficient
        * f64::ln(
            f64::cos(
                f64::atan(
                    initial_speed * (f64::sqrt(air_resistance_coefficient / (mass * GRAVITY))),
                ) - time_air_resistance * (f64::sqrt(GRAVITY * air_resistance_coefficient / mass)),
            ) / f64::cos(f64::atan(
                initial_speed * (f64::sqrt(air_resistance_coefficient / (mass * GRAVITY))),
            )),
        ); //высота

    println!("Время полета = {time_air_resistance} секунд");
    println!("Наибольшая высота подьёма = {height_air_resistance} метров");

    //Пренебрегая сопротивлением
    let time_no_resistance = initial_speed / GRAVITY; // время
    let height_no_resistance = initial_speed * time_no_resistance
        - GRAVITY * time_no_resistance * time_no_resistance / 2.0; // высота

    println!("\n*****************************************");
    println!("Пренебрегая сопротивлением");
    println!("Время полета = {time_no_resistance} секунд");
    println!("Наибольшая высота подьёма = {height_no_resistance} метров");

    stdin().read_line(&mut user_input).unwrap();

    println!("График полёта мяча");

    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| {
            calculate_height(x.into(), initial_speed, air_resistance_coefficient, mass) as f32
        })))
        .lineplot(&Shape::Continuous(Box::new(|x| {
            calculate_height(x.into(), initial_speed, 0.0, mass) as f32
        })))
        .display();
}

fn calculate_height(time: f64, initial_speed: f64, air_resistance_coefficient: f64, mass: f64) -> f64 {
    //println!("{time}");
    // returns height
    if air_resistance_coefficient == 0.0 {
        initial_speed * time - GRAVITY * time * time / 2.0
    } else {
        mass / air_resistance_coefficient
            * f64::ln(
                f64::cos(
                    f64::atan(air_resistance_coefficient * (f64::sqrt(air_resistance_coefficient / (mass * GRAVITY))))
                        - time * (f64::sqrt(GRAVITY * air_resistance_coefficient / mass)),
                ) / f64::cos(f64::atan(
                    air_resistance_coefficient * (f64::sqrt(air_resistance_coefficient / (mass * GRAVITY))),
                )),
            )
    }
}
