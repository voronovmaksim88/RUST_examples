use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let exe_path = std::env::current_exe()?;
    let root_dir = exe_path
        .parent()
        .ok_or_else(|| io::Error::other("Не удалось определить папку exe"))?;

    println!("Сканирую каталог: {}", root_dir.display());

    let mut total_lines = 0usize;
    walk_dir(root_dir, root_dir, &mut total_lines)?;

    println!("\nСуммарное число строк во всех файлах: {total_lines}");
    wait_for_esc()?;
    Ok(())
}

fn walk_dir(root_dir: &Path, dir: &Path, total_lines: &mut usize) -> io::Result<()> {
    for entry_result in fs::read_dir(dir)? {
        let entry = entry_result?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            if name == ".git"
                || name == ".venv"
                || name == "node_modules"
                || name == "__pycache__"
            {
                continue;
            }
            walk_dir(root_dir, &path, total_lines)?;
        } else if file_type.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("xls") || ext.eq_ignore_ascii_case("xlsx") {
                    continue;
                }
            }

            match count_lines_in_file(&path) {
                Ok(lines) => {
                    *total_lines += lines;
                    let shown_path = relative_or_full(root_dir, &path);
                    println!("{shown_path}: {lines}");
                }
                Err(err) => {
                    let shown_path = relative_or_full(root_dir, &path);
                    eprintln!("{shown_path}: ошибка чтения ({err})");
                }
            }
        }
    }

    Ok(())
}

fn count_lines_in_file(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; 8192];
    let mut count = 0usize;
    let mut last_byte: Option<u8> = None;
    let mut file_has_data = false;

    loop {
        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }

        file_has_data = true;
        count += buf[..bytes_read].iter().filter(|&&b| b == b'\n').count();
        last_byte = Some(buf[bytes_read - 1]);
    }

    if file_has_data && last_byte != Some(b'\n') {
        count += 1;
    }

    Ok(count)
}

fn relative_or_full(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(PathBuf::from)
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string()
}

#[cfg(windows)]
fn wait_for_esc() -> io::Result<()> {
    println!("Нажмите Esc для завершения...");
    windows_wait_for_escape_key()
}

#[cfg(not(windows))]
fn wait_for_esc() -> io::Result<()> {
    println!("Press Enter to exit...");
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(())
}

#[cfg(windows)]
fn windows_wait_for_escape_key() -> io::Result<()> {
    use std::ffi::c_void;
    use std::mem::MaybeUninit;

    type Handle = *mut c_void;
    type Bool = i32;
    type Dword = u32;
    type Word = u16;
    type Wchar = u16;

    const STD_INPUT_HANDLE: i32 = -10;
    const KEY_EVENT: Word = 0x0001;
    const VK_ESCAPE: Word = 0x1B;

    #[repr(C)]
    union CharUnion {
        unicode_char: Wchar,
        ascii_char: u8,
    }

    #[repr(C)]
    struct KeyEventRecord {
        b_key_down: Bool,
        w_repeat_count: Word,
        w_virtual_key_code: Word,
        w_virtual_scan_code: Word,
        u_char: CharUnion,
        dw_control_key_state: Dword,
    }

    #[repr(C)]
    union InputRecordEvent {
        key_event: std::mem::ManuallyDrop<KeyEventRecord>,
        _padding: [u8; 16],
    }

    #[repr(C)]
    struct InputRecord {
        event_type: Word,
        _padding: Word,
        event: InputRecordEvent,
    }

    unsafe extern "system" {
        fn GetStdHandle(n_std_handle: i32) -> Handle;
        fn ReadConsoleInputW(
            h_console_input: Handle,
            lp_buffer: *mut InputRecord,
            n_length: Dword,
            lp_number_of_events_read: *mut Dword,
        ) -> Bool;
    }

    let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    if handle.is_null() || handle as isize == -1 {
        return Err(io::Error::last_os_error());
    }

    loop {
        let mut record = MaybeUninit::<InputRecord>::zeroed();
        let mut events_read: Dword = 0;
        let ok = unsafe { ReadConsoleInputW(handle, record.as_mut_ptr(), 1, &mut events_read) };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        if events_read == 0 {
            continue;
        }

        let record = unsafe { record.assume_init() };
        if record.event_type != KEY_EVENT {
            continue;
        }

        let key = unsafe { &record.event.key_event };
        if key.b_key_down != 0 && key.w_virtual_key_code == VK_ESCAPE {
            break;
        }
    }

    Ok(())
}
