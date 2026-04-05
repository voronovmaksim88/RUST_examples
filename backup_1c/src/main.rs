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

const DEFAULT_BACKUP_DEST: &str = r"D:\SynologyDriveVoronov\Копии 1С";
const DEFAULT_SOURCE_DB: &str = r"C:\Users\Maksim\Dropbox\1С бухгалтерия СибПЛК";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
struct Config {
    backup_dest: String,
    source_db: String,
}

impl Config {
    fn new(backup_dest: String, source_db: String) -> Self {
        Self {
            backup_dest,
            source_db,
        }
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

    let backup_dest = get_path(&config_path, "backup_dest");
    let source_db = get_path(&config_path, "source_db");

    println!();
    println!("Путь для бэкапов: {}", backup_dest);
    println!("Источник БД 1С:   {}", source_db);
    println!();
    print!("Нажмите Enter для выхода... ");
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}

fn get_path(config_path: &Path, key: &str) -> String {
    let mut config = Config::load(config_path);

    let default_path = match key {
        "backup_dest" => DEFAULT_BACKUP_DEST,
        "source_db" => DEFAULT_SOURCE_DB,
        _ => unreachable!(),
    };

    let mut path = if let Some(ref cfg) = config {
        match key {
            "backup_dest" => cfg.backup_dest.clone(),
            "source_db" => cfg.source_db.clone(),
            _ => unreachable!(),
        }
    } else {
        default_path.to_string()
    };

    let step_label = match key {
        "backup_dest" => "Шаг 1. Поиск пути хранения бэкапов .... ",
        "source_db" => "Шаг 2. Поиск базы данных 1С .... ",
        _ => unreachable!(),
    };

    let prompt_label = match key {
        "backup_dest" => "Введите новый путь для бэкапов: ",
        "source_db" => "Введите новый путь к базе данных 1С: ",
        _ => unreachable!(),
    };

    loop {
        print!("{}", step_label);
        io::stdout().flush().unwrap();

        if Path::new(&path).exists() {
            println!("{}", "успешно".green());

            let need_save = match &config {
                None => true,
                Some(cfg) => match key {
                    "backup_dest" => cfg.backup_dest != path,
                    "source_db" => cfg.source_db != path,
                    _ => unreachable!(),
                },
            };

            if need_save {
                let (dest, src) = match key {
                    "backup_dest" => (
                        path.clone(),
                        config
                            .as_ref()
                            .map(|c| c.source_db.clone())
                            .unwrap_or_else(|| DEFAULT_SOURCE_DB.to_string()),
                    ),
                    "source_db" => (
                        config
                            .as_ref()
                            .map(|c| c.backup_dest.clone())
                            .unwrap_or_else(|| DEFAULT_BACKUP_DEST.to_string()),
                        path.clone(),
                    ),
                    _ => unreachable!(),
                };
                config = Some(Config::new(dest, src));
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

            print!("{}", prompt_label);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            path = input.trim().to_string();

            if path.is_empty() {
                path = default_path.to_string();
            }
            println!();
        }
    }
}
