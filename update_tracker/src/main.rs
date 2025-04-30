use notify::{Error, Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::env;
use rusqlite::{params, Connection, Result as SqlResult};

#[derive(Debug)]
struct FileEvent {
    event_kind: String,
    path: String,
    rename_from: Option<String>,
    rename_to: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("file_index.db")?;

    conn.execute(
        "create table if not exists file_index (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                event_kind TEXT NOT NULL,
                path TEXT NOT NULL,
                rename_from TEXT,
                rename_to TEXT
            )",
        [],
    )?;

    let mut watcher = notify::recommended_watcher(|res: std::result::Result<Event, Error>| {
        match res {
            Ok(event) => {
                if let Some(path_str) = event.paths.first().and_then(|p| p.to_str()) {
                    
                    let exclude_patterns = [
                        "\\AppData\\",
                        "\\.git",
                        "\\node_modules",
                        "\\.vscode",
                        "\\target",
                        "\\.vs",
                        "\\bin",
                        "\\obj",
                        "\\build",
                        "\\dist"
                    ];
                    
                    if !exclude_patterns.iter().any(|&pattern| path_str.contains(pattern)) {
                        match event.kind {
                            EventKind::Create(_) => {
                                println!("create event");
                            }
                            EventKind::Remove(_) => {
                                println!("delete event");
                            }
                            EventKind::Modify(notify::event::ModifyKind::Name(_)) => {
                                println!("modified name");
                            }
                            _ => {}
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