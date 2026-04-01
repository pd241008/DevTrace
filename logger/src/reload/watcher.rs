use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::sync::mpsc::channel;
use std::process::{Command, Child};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use std::fs;
use std::env;

pub fn start_hot_reload() {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).expect("Failed to init watcher");

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("🔥 Hot Reload Enabled...");

    let mut child: Option<Child> = spawn_server();

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if is_relevant_change(&event) {
                    println!("♻️ Change detected. Restarting server...");

                    if let Some(mut c) = child.take() {
                        let _ = c.kill();
                        let _ = c.wait();
                    }

                    child = spawn_server();
                }
            }
            Ok(Err(e)) => eprintln!("Watch event error: {:?}", e),
            Err(e) => eprintln!("Watch channel error: {:?}", e),
        }
    }
}

fn spawn_server() -> Option<Child> {
    println!("🔨 Building project...");

    let status = Command::new("cargo")
        .args(["build"])
        .status()
        .expect("Build failed");

    if !status.success() {
        println!("❌ Build failed. Skipping restart.");
        return None;
    }

    sleep(Duration::from_millis(200));

   
    let binary_name = get_binary_name();

    let src = get_binary_path(&binary_name);
    let dst = get_temp_binary_path(&binary_name);

    // Copy binary
    if let Err(e) = fs::copy(&src, &dst) {
        eprintln!("❌ Failed to copy binary: {:?}", e);
        return None;
    }

    println!("🚀 Starting server: {:?}", dst);

    Command::new(dst)
        .env("DEVTRACE_RUN", "1")
        .spawn()
        .ok()
}


fn get_binary_name() -> String {
    env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "app".to_string())
}

// 🔥 Build correct OS-specific path
fn get_binary_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from("target/debug");

    if cfg!(windows) {
        path.push(format!("{}.exe", name));
    } else {
        path.push(name);
    }

    path
}

// 🔥 Temp run binary (prevents lock)
fn get_temp_binary_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from("target/debug");

    if cfg!(windows) {
        path.push(format!("{}_run.exe", name));
    } else {
        path.push(format!("{}_run", name));
    }

    path
}

fn is_relevant_change(event: &Event) -> bool {
    event.paths.iter().any(|path| {
        if let Some(p) = path.to_str() {
            if p.contains("target") {
                return false;
            }
        }

        path.extension()
            .map(|ext| ext == "rs")
            .unwrap_or(false)
    })
}