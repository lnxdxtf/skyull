use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use indicatif::{ProgressBar, ProgressStyle};

pub static COMMAND_NEW_STAGES: &[&str] = &[
    "Starting project directory",
    "Adding dependencies",
    "Creating project structure ",
    "Testing project",
    "Done",
];

pub fn indicator() -> Arc<Mutex<ProgressBar>> {
    let pb = Arc::new(Mutex::new(ProgressBar::new_spinner()));
    pb.lock().unwrap().enable_steady_tick(Duration::from_millis(80));
    let style = ProgressStyle::with_template("{spinner:.blue} {msg}")
        .unwrap()
        // For more spinners check out the cli-spinners project:
        // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
        .tick_strings(&["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "]);
    pb.lock().unwrap().set_style(style);
    pb.lock().unwrap().set_message("Starting...");
    pb
}