use notify::{Watcher, RecursiveMode, Result, Error, Event};
use std::path::Path;

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res: std::result::Result<Event, Error>| {
        match res {
            Ok(event) => {
                // Skip events from AppData directory
                if let Some(path_str) = event.paths.first().and_then(|p| p.to_str()) {
                    // Define patterns to exclude
                    
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
                        println!("event: {:?}", event);
                    }
                } else {
                    println!("event: {:?}", event);
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new("C:\\Users\\muham"), RecursiveMode::Recursive)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1)); // Keep the thread alive to watch
    }

    // Ok(())
}
