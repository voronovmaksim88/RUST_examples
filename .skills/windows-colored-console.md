# Windows Colored Console Output (Rust)

## Проблема

ANSI escape-коды (например `\x1b[32m`) не работают в Windows-консоли по умолчанию.
Крейт `colored` сам по себе **не решает** проблему — нужно сначала включить поддержку
виртуального терминала через WinAPI.

## Решение

### 1. Зависимости в `Cargo.toml`

```toml
[dependencies]
colored = "3"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["consoleapi", "processenv", "winbase", "handleapi"] }
```

### 2. Импорт в `main.rs`

```rust
use colored::Colorize;

#[cfg(windows)]
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
```

### 3. Функция включения ANSI

```rust
#[cfg(windows)]
fn enable_ansi_support() {
    unsafe {
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        let stderr = GetStdHandle(STD_ERROR_HANDLE);

        let mut mode: u32 = 0;
        if GetConsoleMode(stdout, &mut mode) != 0 {
            SetConsoleMode(stdout, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }

        let mut mode: u32 = 0;
        if GetConsoleMode(stderr, &mut mode) != 0 {
            SetConsoleMode(stderr, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

#[cfg(not(windows))]
fn enable_ansi_support() {
    // На Unix-системах цветной вывод работает по умолчанию
}
```

### 4. Вызов в начале `main()`

```rust
fn main() {
    enable_ansi_support();  // <-- ОБЯЗАТЕЛЬНО первым вызовом

    // дальше весь цветной вывод будет работать
    println!("{}", "успешно".green());
    println!("{}", "ошибка".red());
    println!("{}", "предупреждение".yellow());
}
```

## Доступные цвета (colored crate)

```rust
"текст".black()
"текст".red()
"текст".green()
"текст".yellow()
"текст".blue()
"текст".magenta()
"текст".cyan()
"текст".white()
"текст".bright_black()
"текст".bright_red()
"текст".bright_green()
"текст".bright_yellow()
"текст".bright_blue()
"текст".bright_magenta()
"текст".bright_cyan()
"текст".bright_white()
"текст".bold()
```

## Цепочка методов

```rust
println!("{}", "Настройки связи".cyan().bold());
println!("{}", "ОШИБКА".red().bold());
println!("{}", format!("Порт: {}", port).green());
```

## Важные правила

1. `enable_ansi_support()` вызывать **самым первым** в `main()`
2. Не использовать сырые ANSI-коды (`\x1b[32m`) — использовать методы `colored`
3. `colored` + `winapi` — минимальный набор для цветного вывода в Windows
4. Условная компиляция `#[cfg(windows)]` обязательна — иначе не соберётся на Linux/macOS
