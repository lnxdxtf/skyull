use owo_colors::OwoColorize;

use crate::modules::cli::analyze::indicator::indicator;

use super::indicator::COMMAND_NEW_STAGES;

/// Get the time when the function was called
/// Calculate the time when the functions is done
/// And print the total time to execute
pub fn get_time_exec<F>(func: F)
where
    F: Fn(),
{
    let time_init = std::time::Instant::now();
    func();
    let total_time_exec = std::time::Instant::now() - time_init;
    let msg_timer = format!("{}  {:?}", COMMAND_NEW_STAGES[4].bold().bright_green(), total_time_exec.bold().green());
    indicator().lock().unwrap().finish_with_message(msg_timer);
}
