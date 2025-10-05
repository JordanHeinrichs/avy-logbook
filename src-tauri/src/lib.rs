#[derive(serde::Serialize)]
struct Log {
    id: i32,
    name: String,
    date: String,
}

#[tauri::command]
fn log_list() -> Result<Vec<Log>, String> {
    Ok(vec![
        Log {
            id: 3,
            name: String::from("Mount Hector"),
            date: String::from("2025-01-02"),
        },
        Log {
            id: 2,
            name: String::from("Little Sifton"),
            date: String::from("2024-12-30"),
        },
        Log {
            id: 1,
            name: String::from("Ursus Trees"),
            date: String::from("2024-12-29"),
        },
    ])
}

#[tauri::command]
fn create_log(name: String, date: String) -> Result<Log, String> {
    println!("Creating log entry with the name: {}, date: {}", name, date);
    Ok(Log {
        id: 3,
        name: String::from("Grand Daddy"),
        date: String::from("2025-01-03"),
    })
}

#[tauri::command]
fn fetch_log(id: i32) -> Result<Log, String> {
    match id {
        1 => Ok(Log {
            id: 1,
            name: String::from("Ursus Trees"),
            date: String::from("2024-12-29"),
        }),
        2 => Ok(Log {
            id: 2,
            name: String::from("Little Sifton"),
            date: String::from("2024-12-30"),
        }),
        3 => Ok(Log {
            id: 3,
            name: String::from("Mount Hector"),
            date: String::from("2025-01-02"),
        }),
        4 => Ok(Log {
            id: 4,
            name: String::from("Grand Daddy"),
            date: String::from("2025-01-03"),
        }),
        _ => Err(String::from("Unable to find log entry")),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![log_list, create_log, fetch_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
