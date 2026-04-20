use serde::Deserialize;
use std::net::ToSocketAddrs;
use tokio_modbus::prelude::*;

/// Параметры из полей ввода (Modbus TCP unit id + хост + порт).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusTcpParams {
    pub device_address: u8,
    pub tcp_host: String,
    pub tcp_port: u16,
}

/// Читает input registers по адресу 0, 2×u16 → i32 (как `process_register_data` для i32 в RUST-CLI-ModBus-Master).
#[tauri::command]
async fn read_uptime_register(params: ModbusTcpParams) -> Result<String, String> {
    let socket_addr = format!(
        "{}:{}",
        params.tcp_host.trim(),
        params.tcp_port
    )
    .to_socket_addrs()
    .map_err(|e| format!("Ошибка разрешения адреса: {e}"))?
    .next()
    .ok_or_else(|| "Не удалось получить адрес сокета".to_string())?;

    let mut ctx = tokio_modbus::client::tcp::connect(socket_addr)
        .await
        .map_err(|e| format!("TCP: {e:?}"))?;

    ctx.set_slave(Slave(params.device_address));

    let words = ctx
        .read_input_registers(0, 2)
        .await
        .map_err(|e| format!("Modbus: {e:?}"))?;

    if words.len() < 2 {
        return Err("Недостаточно данных (ожидалось 2 регистра)".into());
    }

    let low = words[0] as u32;
    let high = words[1] as u32;
    let combined = (high << 16) | low;
    Ok(format!("{}", combined as i32))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_uptime_register])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
