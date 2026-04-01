use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::sync::mpsc::channel;
use std::process::{Command, Child};
use std::path::Path;

fn main() {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).expect("Failed to init watcher");

    watcher
        .watch(Path::new("../logger/src"), RecursiveMode::Recursive)
        .expect("Failed to watch logger src");

    println!("🔥 DevTrace Runner Started...");

    let mut child: Option<Child> = build_and_run();

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if is_rs_file(&event) {
                    println!("♻️ Change detected. Rebuilding...");

                    if let Some(mut c) = child.take() {
                        let _ = c.kill();
                        let _ = c.wait();
                    }

                    child = build_and_run();
                }
            }
            Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
            Err(e) => eprintln!("Channel error: {:?}", e),
        }
    }
}

fn build_and_run() -> Option<Child> {
    println!("🔨 Building logger...");

    let status = Command::new("cargo")
        .current_dir("../logger")
        .args(["build"])
        .status()
        .expect("Build failed");

    if !status.success() {
        println!("❌ Build failed");
        return None;
    }



    let binary = if cfg!(windows) {
        "../logger/target/debug/logger.exe"
    } else {
        "../logger/target/debug/logger"
    };

    Command::new(binary)
        .spawn()
        .ok()
}

fn is_rs_file(event: &Event) -> bool {
    event.paths.iter().any(|p| {
        p.extension()
            .map(|ext| ext == "rs")
            .unwrap_or(false)
    })
}