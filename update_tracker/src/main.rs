use notify::{Error, Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::env;
use rusqlite::{params, Connection, Result as SqlResult};
use std::sync::{Arc, Mutex};
use notify::event::RenameMode;

#[derive(Debug)]
struct FileEvent {
    event_kind: String,
    path: String,
    extension: String
}

fn get_file_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string()
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("file_index.db")?;

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

    let mut watcher = notify::recommended_watcher(move |res: std::result::Result<Event, Error>| {
        match res {
            Ok(event) => {
                if let Some(path_str) = event.paths.first().and_then(|p| p.to_str()) {
                    
                    let exclude_patterns = [
                        "\\AppData\\",
                        "\\$Recycle.Bin",
                        "\\System Volume Information",
                        "\\.cache",
                        "\\Temp\\",
                        "\\tmp\\",
                        "\\Windows\\",
                        "\\ProgramData\\",  

                        "\\.git",
                        "\\.svn",
                        "\\.hg",
                        "\\node_modules",
                        "\\.vscode",
                        "\\.idea",
                        "\\.vs",
                        "\\target",
                        "\\bin",
                        "\\obj",
                        "\\build",
                        "\\dist",
                        "\\out",
                        "\\Debug",
                        "\\Release",

                        "\\.npm",
                        "\\.yarn",
                        "\\.cargo",
                        "\\packages",

                        "file_index.db",
                        ".db-journal",
                        ".db-shm",
                        ".db-wal",
                        ".sqlite-journal",
                        ".sqlite-shm",
                        ".sqlite-wal",

                         "\\Cache\\",
                        "\\CacheStorage\\"
                    ];

                    let ignored_extensions = [
                        "tmp", "temp", "swp", "swo", "bak", "~", "part",
                        
                        "lock",
                        
                        "log",
                        
                        "db", "sqlite", "db-journal", "db-shm", "db-wal",
                        
                        "pyc", "pyo", "obj", "o", "a", "lib", "so", "dll",
                        
                        "zip", "tar", "gz", "7z", "rar",
                        
                        "sys", "ini"
                    ];

                    let extension = get_file_extension(path_str);
                    
                    if !exclude_patterns.iter().any(|&pattern| path_str.contains(pattern)) && !ignored_extensions.contains(&extension.as_str()){
                        let file_event = match event.kind {
                            EventKind::Create(_) => {
                                println!("create event: {}", path_str);
                                Some(FileEvent {
                                    event_kind: "create".to_string(),
                                    path: path_str.to_string(),
                                    extension
                                })
                            }
                            EventKind::Remove(_) => {
                                println!("delete event: {}", path_str);
                                Some(FileEvent {
                                    event_kind: "delete".to_string(),
                                    path: path_str.to_string(),
                                    extension
                                })
                            }
                            EventKind::Modify(notify::event::ModifyKind::Name(rename_mode)) => {
                                match rename_mode {
                                    RenameMode::From => {
                                        println!("rename from: {}", path_str);
                                        Some(FileEvent { event_kind: "rename_from".to_string(), path: path_str.to_string(), extension })
                                    }
                                    RenameMode::To => {
                                        println!("rename to: {}", path_str);
                                        Some(FileEvent { event_kind: "rename_to".to_string(), path: path_str.to_string(), extension })
                                    }
                                    RenameMode::Both => {
                                        println!("rename both: {}", path_str);
                                        Some(FileEvent { event_kind: "rename_both".to_string(), path: path_str.to_string(), extension })
                                    }
                                    _ => {
                                        println!("unspecified");
                                        Some(FileEvent { 
                                            event_kind: "rename".to_string(),
                                            path: path_str.to_string(),
                                            extension
                                        })
                                    }
                                }
                            }
                            _ => None
                        };

                        if let Some(event) = file_event {
                            if let Ok(conn) = conn_clone.lock() {
                                match conn.execute(
                                    "INSERT INTO file_index (event_kind, path, extension) VALUES (?1, ?2, ?3)",
                                    params![event.event_kind, event.path, event.extension],
                                ) {
                                    Ok(_) => println!("Event saved to database"),
                                    Err(e) => eprintln!("Failed to save to database: {}", e),
                                }
                            } else {
                                eprintln!("Failed to acquire database lock");
                            }
                        }
                    }
                } else {
                    println!("else statement")
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).expect("Could not find home directory");
    watcher.watch(Path::new(&home_dir), RecursiveMode::Recursive)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}