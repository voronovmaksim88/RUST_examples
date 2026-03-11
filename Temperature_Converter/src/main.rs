use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    input.trim().to_string()
}

fn read_temperature(prompt: &str) -> f32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<f32>() {
            Ok(value) => return value,
            Err(_) => println!("Введите корректное число."),
        }
    }
}

fn read_menu_choice() -> String {
    loop {
        let choice = read_input("Выберите, что хотите конвертировать:\n(1) °C -> °F\n(2) °F -> °C");
        match choice.as_str() {
            "1" | "2" => return choice,
            _ => println!("Неверный выбор. Введите 1 или 2."),
        }
    }
}

fn convert_c_to_f() {
    let c = read_temperature("Введите градусы Цельсия:");
    let f = c * 9.0 / 5.0 + 32.0;
    println!("{:.2}°C = {:.2}°F", c, f);
}

fn convert_f_to_c() {
    let f = read_temperature("Введите градусы Фаренгейта:");
    let c = (f - 32.0) * 5.0 / 9.0;
    println!("{:.2}°F = {:.2}°C", f, c);
}

fn main() {
    let answer = read_menu_choice();

    match answer.as_str() {
        "1" => convert_c_to_f(),
        "2" => convert_f_to_c(),
        _ => unreachable!(),
    }
}
