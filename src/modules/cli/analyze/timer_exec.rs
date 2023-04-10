use owo_colors::OwoColorize;

pub fn get_time_exec<F>(func: F)
where
    F: Fn(),
{
    let time_init = std::time::Instant::now();
    func();
    let total_time_exec = std::time::Instant::now() - time_init;
    println!("Total Time To Exec: {:?}", total_time_exec.bold().green());
}
