use std::io;

fn read_temperature(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    input.trim().parse().expect("Введите корректное число")
}

fn convert_c_to_f() {
    let c = read_temperature("Введите градусы Цельсия:");
    let f = c * 9.0 / 5.0 + 32.0;
    println!("{}°C = {}°F", c, f);
}

fn convert_f_to_c() {
    let f = read_temperature("Введите градусы Фаренгейта:");
    let c = (f - 32.0) * 5.0 / 9.0;
    println!("{}°F = {}°C", f, c);
}

fn main() {
    let mut answer = String::new();
    println!("Выберетите что хотите конвертировать");
    println!("(1) С° в F°");
    println!("(2) F° в C°");

    io::stdin()
        .read_line(&mut answer)
        .expect("не удалось считать строку ");

    match answer.trim() {
        "1" => {
            convert_c_to_f();
            // ... ваш код для конвертации C -> F
        }
        "2" => {
            convert_f_to_c();
            // ... ваш код для конвертации F -> C
        }
        _ => println!("Неверный выбор!"),
    }
}
