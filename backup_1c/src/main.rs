use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[cfg(windows)]
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

const DEFAULT_BACKUP_PATH: &str = r"D:\SynologyDriveVoronov\Копии 1С";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
struct Config {
    backup_path: String,
}

impl Config {
    fn new(backup_path: String) -> Self {
        Self { backup_path }
    }

    fn load(path: &Path) -> Option<Self> {
        if path.exists() {
            let content = fs::read_to_string(path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn save(&self, path: &Path) -> io::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}

fn get_exe_dir() -> PathBuf {
    env::current_exe()
        .expect("Не удалось определить путь к exe")
        .parent()
        .expect("Не удалось получить родительскую директорию")
        .to_path_buf()
}

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
fn enable_ansi_support() {}

fn main() {
    enable_ansi_support();

    println!("Создание резервной копии БД 1С");
    println!();

    let exe_dir = get_exe_dir();
    let config_path = exe_dir.join(CONFIG_FILENAME);

    let backup_path = get_backup_path(&config_path);

    println!();
    println!("Путь для бэкапов: {}", backup_path);
    println!();
    print!("Нажмите Enter для выхода... ");
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}

fn get_backup_path(config_path: &Path) -> String {
    let mut config = Config::load(config_path);

    let mut path = if let Some(ref cfg) = config {
        cfg.backup_path.clone()
    } else {
        DEFAULT_BACKUP_PATH.to_string()
    };

    loop {
        print!("Шаг 1. Поиск пути хранения бэкапов .... ");
        io::stdout().flush().unwrap();

        if Path::new(&path).exists() {
            println!("{}", "успешно".green());

            if config.as_ref().map_or(true, |c| c.backup_path != path) {
                config = Some(Config::new(path.clone()));
                if let Some(ref cfg) = config {
                    cfg.save(config_path)
                        .expect("Не удалось сохранить config.json");
                }
            }

            return path;
        } else {
            println!("{}", "не найдено".red());
            println!();
            println!("Путь не найден: {}", path);

            print!("Введите новый путь для бэкапов: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            path = input.trim().to_string();

            if path.is_empty() {
                path = DEFAULT_BACKUP_PATH.to_string();
            }
            println!();
        }
    }
}
