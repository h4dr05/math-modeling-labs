use std::io::stdin;
use std::io::{self, Write};
use textplots::{Chart, Plot, Shape};

fn main() {
    std::env::set_var("LC_ALL", "ru_RU.UTF-8");

    let g = 9.80665; // ускорение свободного падения
    let mut m: f64; // масса сопротивления
    let k: f64; // коэффициент сопротивления
    let c: f64; // начальная скорость

    print!("Введите вес мяча в кГ = ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    m = input.trim().parse().expect("Error parsing input");

    m = m / g; // рассчёт массы

    input.clear();
    print!("Введите сопротивление воздуха в кГ = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    k = input.trim().parse().expect("Error parsing input");

    input.clear();
    print!("Введите начальную скорость мяча = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    c = input.trim().parse().expect("Error parsing input");

    println!("*****************************************");

    let t = f64::sqrt(m / (g * k)) * f64::atan(c * (f64::sqrt(k / (m * g)))); //время
    let s = m / k
        * f64::ln(
            f64::cos(f64::atan(c * (f64::sqrt(k / (m * g)))) - t * (f64::sqrt(g * k / m)))
                / f64::cos(f64::atan(c * (f64::sqrt(k / (m * g))))),
        ); //высота

    println!("Время полета = {t} секунд");
    println!("Наибольшая высота подьёма = {s} метров");

    //Пренебрегая сопротивлением
    let t1 = c / g; // время
    let s1 = c * t1 - g * t1 * t1 / 2.0; // высота

    println!("\n*****************************************");
    println!("Пренебрегая сопротивлением");
    println!("Время полета = {t1} секунд");
    println!("Наибольшая высота подьёма = {s1} метров");

    stdin().read_line(&mut input).unwrap();

    println!("График полёта мяча");

    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x)))
        .display();
}

fn height(t1: f64, s1: f64) {

}