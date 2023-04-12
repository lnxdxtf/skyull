use owo_colors::OwoColorize;

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
    println!("Total Time To Exec: {:?}", total_time_exec.bold().green());
}
