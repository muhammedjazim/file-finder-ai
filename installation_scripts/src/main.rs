use std::fs;
use std::time::Instant;
use std::env;
use helper_library::{utils, file_event};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let root = env::var("HOME").or_else(|_| env::var("USERPROFILE")).expect("Could not find home directory");
    let mut stack = vec![root];

    let data_dir = utils::get_app_data_dir();
    let db_path = data_dir.join("file_index.db");
    println!("Using database at: {}", db_path.display());
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "create table if not exists file_index (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                event_kind TEXT NOT NULL,
                path TEXT NOT NULL,
                extension TEXT NOT NULL
            )",
        [],
    )?;

    let conn = Arc::new(Mutex::new(conn));
    let conn_clone = Arc::clone(&conn);

    let exclude_patterns = [
        "\\AppData\\", "\\$Recycle.Bin", "\\System Volume Information", "\\.cache", "\\Temp\\",
        "\\tmp\\", "\\Windows\\", "\\ProgramData\\", "\\.git", "\\.svn", "\\.hg", "\\node_modules",
        "\\.vscode", "\\.idea", "\\.vs", "\\target", "\\bin", "\\obj", "\\build", "\\dist", "\\out",
        "\\Debug", "\\Release", "\\.npm", "\\.yarn", "\\.cargo", "\\packages", "file_index.db", 
        ".db-journal", ".db-shm", ".db-wal", ".sqlite-journal", ".sqlite-shm", ".sqlite-wal", 
        "\\Cache\\", "\\CacheStorage\\"
    ];

    let ignored_extensions = [
        "tmp", "temp", "swp", "swo", "bak", "~", "part", "lock", "log", "db", "sqlite", "db-journal", 
        "db-shm", "db-wal", "pyc", "pyo", "obj", "o", "a", "lib", "so", "dll", "zip", "tar", "gz", 
        "7z", "rar", "sys", "ini"
    ];

    while let Some(current) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current) {
            for entry in entries.flatten() {
                let path = entry.path();
                let path_str = path.display().to_string();

                if utils::is_filepath_or_extension_ignored(&path_str, &exclude_patterns, &ignored_extensions) {
                    continue;
                }

                let extension = utils::get_file_extension(&path_str);

                let event = file_event::FileEvent {
                    event_kind: "initial_scan".to_string(),
                    path: path_str.clone(),
                    extension,
                };

                println!("Name: {}", path.display());

                if let Ok(conn) = conn_clone.lock() {
                    if let Err(e) = conn.execute(
                        "INSERT INTO file_index (event_kind, path, extension) VALUES (?1, ?2, ?3)",
                        params![event.event_kind, event.path, event.extension],
                    ) {
                        eprintln!("Failed to insert: {}", e);
                    }
                } else {
                    eprintln!("Failed to acquire DB lock");
                }

                if path.is_dir() {
                    stack.push(path.display().to_string());
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("\nTime taken: {:.2?}", duration);

    Ok(())
}
